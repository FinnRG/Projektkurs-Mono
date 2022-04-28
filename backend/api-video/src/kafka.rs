use std::time::Duration;

use crate::{get_conn, Video, Visibility};
use log::{info, warn};
use rdkafka::{
    config::RDKafkaLogLevel,
    consumer::{Consumer, StreamConsumer},
    message::{Header, Headers, OwnedHeaders},
    producer::{future_producer::OwnedDeliveryResult, FutureProducer, FutureRecord},
    ClientConfig, Message, Offset,
};
use redis::Commands;
use strum::IntoStaticStr;

#[derive(IntoStaticStr, PartialEq, Clone, Copy)]
pub enum VideoEvents {
    Created,
    Changed,
    Deleted,
}

// Publishes a VideoCreated event to videos
pub async fn emit_video(id: &str, video: &str, event: VideoEvents) -> OwnedDeliveryResult {
    let producer: &FutureProducer = &ClientConfig::new()
        .set(
            "bootstrap.servers",
            "kafka-headless.default.svc.cluster.local:9092",
        )
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    let mut record = FutureRecord::to("videos")
        .key(id)
        .headers(OwnedHeaders::new().insert(Header {
            key: "type",
            value: Some(event.into()),
        }));

    if event != VideoEvents::Deleted {
        record = record.payload(video);
    }

    producer.send(record, Duration::from_secs(0)).await
}

pub async fn receive_events() {
    info!("Starting to construct Stream Consumer");
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "api-video-consumer")
        .set("bootstrap.servers", "kafka:9092")
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "false")
        .set("auto.offset.reset", "earliest")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create()
        .expect("Consumer creation failed");

    if let Err(e) = consumer.subscribe(&["videos"]) {
        warn!("Kafka subscription error: {}", e);
    }

    if let Err(e) = consumer.seek("videos", 0, Offset::Beginning, None) {
        warn!("Kafka playback error: {}", e);
    }

    let mut conn = get_conn().expect("Unable to create redis connection");

    loop {
        match consumer.recv().await {
            Ok(m) => {
                let payload = match m.payload_view::<str>() {
                    None => "",
                    Some(Ok(s)) => s,
                    Some(Err(_)) => {
                        warn!("Error while deserializing message payload");
                        ""
                    }
                };
                if let Some(headers) = m.headers() {
                    for header in headers.iter() {
                        if header.key == "type" && header.value == Some("Deleted".as_bytes()) {
                            if let Err(e) = conn.del::<_, ()>(m.key()) {
                                warn!("Unable to delete {:?} because of {}", m.key(), e);
                            }
                        } else if header.key == "type" {
                            let mut video: Video = serde_json::from_str(payload)
                                .expect("Unable to deserialize payload");
                            if header.value == Some("Processed".as_bytes()) {
                                video.visibility = Visibility::Unspecified as i32;
                            }
                            if let Err(e) = conn.set::<_, _, ()>(&video.id, payload) {
                                warn!(
                                    "Unable to set {:?} to {} because of {}",
                                    &video.id, payload, e
                                )
                            }
                        }
                    }
                }
            }
            Err(e) => warn!("Kafka error: {}", e),
        }
    }
}
