use std::time::Duration;

use crate::{storage::Store, videos::v1::Status as VideoStatus, Video};
use log::{info, warn};
use rdkafka::{
    config::RDKafkaLogLevel,
    consumer::{Consumer, StreamConsumer},
    message::{BorrowedMessage, Header, Headers, OwnedHeaders},
    producer::{future_producer::OwnedDeliveryResult, FutureProducer, FutureRecord},
    ClientConfig, Message, Offset,
};
use strum::IntoStaticStr;
use tokio::runtime::Runtime;

#[derive(IntoStaticStr, PartialEq, Clone, Copy)]
pub enum VideoEvent {
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
pub async fn emit_video_event(id: &str, video: &str, event: VideoEvent) -> OwnedDeliveryResult {
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

    if event != VideoEvent::Deleted {
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
    let mut store = Store::new();
    let header =
        std::str::from_utf8(header.value.expect("Type header should have a value")).unwrap();
    let key = std::str::from_utf8(m.key().expect("Message should have a key")).unwrap();
    match header {
        "Deleted" => {
            store.del_video(key);
        }
        "Processed" => {
            let payload = stringify_payload(m);
            update_status(&mut store, key, "Finished");
            let rt = Runtime::new().unwrap();
            rt.block_on(emit_video_event(key, payload, VideoEvent::Finished))
                .expect("Unable to create video event");
        }
        "Uploaded" => update_status(&mut store, key, header),
        "TitleChanged" | "DescriptionChanged" | "VisibilityChanged" => {
            let payload = stringify_payload(m);
            let video: Video = Video::from(payload);
            store.set_video(&video);
        }
        _ => {}
    };
}

fn update_status(store: &mut Store, id: &str, status: &str) {
    match store.get_video(id) {
        Ok(mut video) => {
            video.set_status(VideoStatus::from(status));
            store.set_video(&video);
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
