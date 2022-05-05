use std::time::Duration;

use crate::{get_conn, videos::v1::Status as VideoStatus, Video};
use log::{info, warn};
use r2d2::PooledConnection;
use rdkafka::{
    config::RDKafkaLogLevel,
    consumer::{Consumer, StreamConsumer},
    message::{BorrowedMessage, Header, Headers, OwnedHeaders},
    producer::{future_producer::OwnedDeliveryResult, FutureProducer, FutureRecord},
    ClientConfig, Message, Offset,
};
use redis::Commands;
use strum::IntoStaticStr;

#[derive(IntoStaticStr, PartialEq, Clone, Copy)]
pub enum VideoEvents {
    Created,
    Deleted,
    TitleChanged,
    DescriptionChanged,
    VisibilityChanged,
    Uploaded,
    Processed,
    Finished,
}

// Publishes a VideoCreated event to videos
pub async fn emit_video_event(id: &str, video: &str, event: VideoEvents) -> OwnedDeliveryResult {
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

    loop {
        match consumer.recv().await {
            Ok(m) => {
                process_message(&m);
            }
            Err(e) => warn!("Kafka error: {}", e),
        }
    }
}

fn process_message(m: &BorrowedMessage) {
    if let Some(headers) = m.headers() {
        match headers.iter().find(|h| h.key == "type") {
            Some(header) => process_valid_message(m, &header),
            None => (),
        }
    }
}

fn process_valid_message(m: &BorrowedMessage, header: &Header<&[u8]>) {
    let mut conn = get_conn().expect("Unable to create redis connection");
    if header.value == Some("Deleted".as_bytes()) {
        redis_del_video(&mut conn, m.key())
    } else if header.value == Some("Processed".as_bytes())
        || header.value == Some("Uploaded".as_bytes())
    {
        update_status(
            &mut conn,
            m.key(),
            std::str::from_utf8(header.value.unwrap()).unwrap(),
        );
    } else {
        let payload = stringify_payload(m);
        let video: Video = serde_json::from_str(payload).expect("Unable to deserialize payload");
        redis_set_video(&mut conn, &video, payload);
    }
}

#[deprecated]
fn redis_del_video(conn: &mut PooledConnection<redis::Client>, id: Option<&[u8]>) {
    if let Err(e) = conn.del::<_, ()>(id) {
        warn!("Unable to delete {:?} because of {}", id, e);
    }
}

#[deprecated]
fn redis_set_video(conn: &mut PooledConnection<redis::Client>, video: &Video, payload: &str) {
    if let Err(e) = conn.set::<_, _, ()>(&video.id, payload) {
        warn!(
            "Unable to set {:?} to {} because of {}",
            &video.id, payload, e
        )
    }
}

fn update_status(conn: &mut PooledConnection<redis::Client>, id: Option<&[u8]>, status: &str) {
    match conn.get::<_, String>(id) {
        Ok(video) => {
            let mut video: Video =
                serde_json::from_str(&video).expect("Unable to deserialize video");
            let status = VideoStatus::from_str(status);
            video.set_status(status);
            redis_set_video(conn, &video, &serde_json::to_string(&video).unwrap());
        }
        Err(e) => warn!("Unable to get {:?} because of {:?}", id, e),
    }
}

fn stringify_payload<'a>(m: &'a BorrowedMessage) -> &'a str {
    match m.payload_view::<str>() {
        None => "",
        Some(Ok(s)) => s,
        Some(Err(_)) => {
            warn!("Error while deserializing message payload");
            ""
        }
    }
}
