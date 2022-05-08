use std::time::Duration;

use crate::{
    storage::Store, videos::v1::Status as VideoStatus, videos::v1::VideoFinishedEvent, Video,
};
use log::{info, warn};
use rdkafka::{
    config::RDKafkaLogLevel,
    consumer::{Consumer, StreamConsumer},
    message::{BorrowedMessage, Header, Headers, OwnedHeaders},
    producer::{future_producer::OwnedDeliveryResult, FutureProducer, FutureRecord},
    ClientConfig, Message, Offset,
};

use events::VideoEvent;

pub mod events;
mod handler;

// Publishes a VideoCreated event to videos
pub async fn emit_video_event(id: &str, event: VideoEvent) -> OwnedDeliveryResult {
    let producer: &FutureProducer = &ClientConfig::new()
        .set(
            "bootstrap.servers",
            "kafka-headless.default.svc.cluster.local:9092",
        )
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    let bytes = event.as_bytes();
    let event_str: &'static str = event.into();

    let mut record = FutureRecord::to("videos")
        .key(id)
        .headers(OwnedHeaders::new().insert(Header {
            key: "type",
            value: Some(event_str),
        }));

    record = record.payload(&bytes);

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
    warn!("Partition assignment: {:?}", consumer.assignment());

    if let Err(e) = consumer.subscribe(&["videos"]) {
        warn!("Kafka subscription error: {}", e);
    }

    if let Err(e) = consumer.seek("videos", 0, Offset::Beginning, None) {
        warn!("Kafka playback error: {}", e);
    }

    loop {
        match consumer.recv().await {
            Ok(m) => {
                warn!("New message: {:?}", m.offset());
                process_message(&m).await;
            }
            Err(e) => warn!("Kafka error: {}", e),
        }
    }
}

async fn process_message(m: &BorrowedMessage<'_>) {
    if let Some(headers) = m.headers() {
        match headers.iter().find(|h| h.key == "type") {
            Some(header) => process_valid_message(m, &header).await,
            None => (),
        }
    }
}

async fn process_valid_message(m: &BorrowedMessage<'_>, header: &Header<'_, &[u8]>) {
    let header =
        std::str::from_utf8(header.value.expect("Type header should have a value")).unwrap();
    let key = std::str::from_utf8(m.key().expect("Message should have a key")).unwrap();
    handler::handle_event(header, key, m).await;
}
