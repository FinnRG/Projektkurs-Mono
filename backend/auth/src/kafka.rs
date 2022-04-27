use crate::{get_conn, User};
use log::{info, warn};
use rdkafka::consumer::Consumer;
use rdkafka::{
    config::RDKafkaLogLevel,
    consumer::{BaseConsumer, StreamConsumer},
    message::{Header, Headers, OwnedHeaders},
    producer::{future_producer::OwnedDeliveryResult, FutureProducer, FutureRecord},
    ClientConfig, Message, Offset,
};
use redis::Commands;
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
                            let user: User = serde_json::from_str(payload)
                                .expect("Unable to deserialize payload");
                            if let Err(e) = conn.set::<_, _, ()>(user.email, payload) {
                                warn!(
                                    "Unable to set {:?} to {} because of {}",
                                    user.email, user.password, e
                                );
                            }
                        }
                    }
                }
            }
            Err(e) => warn!("Kafka, error {}", e),
        }
    }
}
