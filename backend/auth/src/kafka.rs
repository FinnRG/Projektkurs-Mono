use log::warn;
use rdkafka::{
    message::{OwnedHeaders, Headers, Header},
    producer::{future_producer::OwnedDeliveryResult, FutureProducer, FutureRecord},
    ClientConfig, consumer::StreamConsumer, Message, config::RDKafkaLogLevel,
};
use std::time::Duration;
use strum::IntoStaticStr;
use rdkafka::consumer::Consumer;
use crate::get_conn;
use redis::Commands;

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

    producer.send(record, Duration::from_secs(0)).await
}

pub async fn receive_events() {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "auth-consumer")
        .set("bootstrap.servers", "kafka:9092")
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create()
        .expect("Consumer creation failed");

    consumer.subscribe(&["users"])
        .expect("Can't subscribe to users topic");

    let mut conn = get_conn().expect("Unable to create redis connection");

    loop {
        match consumer.recv().await {
            Err(e) => warn!("Kafka, error: {}", e),
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
                            if let Err(e) = conn.set::<_, _, ()>(m.key(), payload) {
                                warn!("Unable to set {:?} to {} because of {}", m.key(), payload, e);
                            }
                        }
                    }
                }
            }
        }
    }
}
