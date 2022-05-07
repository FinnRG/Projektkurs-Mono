use crate::storage::Store;
use crate::User;
use log::{info, warn};
use rdkafka::consumer::Consumer;
use rdkafka::message::BorrowedMessage;
use rdkafka::{
    config::RDKafkaLogLevel,
    consumer::StreamConsumer,
    message::{Header, Headers, OwnedHeaders},
    producer::{future_producer::OwnedDeliveryResult, FutureProducer, FutureRecord},
    ClientConfig, Message, Offset,
};
use std::time::Duration;
use strum::IntoStaticStr;

#[derive(IntoStaticStr, PartialEq, Clone, Copy)]
pub enum UserEvents {
    Registered,
    Changed,
    Deleted,
}

pub async fn emit_user(id: &str, user: &str, event: UserEvents) -> OwnedDeliveryResult {
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", "kafka:9092")
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    let event_str: &'static str = event.into();

    let mut record = FutureRecord::to("users")
        .key(id)
        .headers(OwnedHeaders::new().insert(Header {
            key: "type",
            value: Some(event_str),
        }));

    if event != UserEvents::Deleted {
        record = record.payload(user);
    }

    let res = producer.send(record, Duration::from_secs(0)).await;
    println!("{}", res.as_ref().unwrap().0);
    res
}

pub async fn receive_events() {
    info!("Starting to construct Stream Consumer");
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "auth-consumer")
        .set("bootstrap.servers", "kafka:9092")
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "false")
        .set("auto.offset.reset", "earliest")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create()
        .expect("Consumer creation failed");

    if let Err(e) = consumer.subscribe(&["users"]) {
        warn!("Kafka subscription error: {}", e);
    }

    if let Err(e) = consumer.seek("users", 0, Offset::Beginning, None) {
        warn!("Kafka playback error: {}", e);
    }

    loop {
        match consumer.recv().await {
            Ok(m) => process_event(&m).await,
            Err(e) => warn!("Kafka, error {}", e),
        }
    }
}

async fn process_event(msg: &BorrowedMessage<'_>) {
    let mut store = Store::new();
    let payload = extract_payload(msg);
    let t = extract_event_type(msg);
    let key = extract_key(msg);

    if key.is_none() {
        return;
    }

    match (t, payload) {
        (Some("Deleted"), None) => {
            //TODO: Fix this
            store.del_user(key.unwrap());
        }
        (Some("Registered"), Some(p)) | (Some("Changed"), Some(p)) => {
            store.set_user(&User::from_json(p));
        }
        _ => {}
    };
}

// TODO: Error logging here
fn extract_payload<'a>(msg: &'a BorrowedMessage) -> Option<&'a str> {
    match msg.payload_view::<str>() {
        Some(Ok(s)) => Some(s),
        _ => None,
    }
}

// TODO: Error logging here
fn extract_key<'a>(msg: &'a BorrowedMessage) -> Option<&'a str> {
    match msg.key_view::<str>() {
        Some(Ok(s)) => Some(s),
        _ => None,
    }
}

fn extract_event_type<'a>(msg: &'a BorrowedMessage) -> Option<&'a str> {
    if let Some(headers) = msg.headers() {
        for header in headers.iter() {
            if header.key == "type" && header.value.is_some() {
                return std::str::from_utf8(header.value.unwrap()).ok();
            }
        }
    }
    None
}
