use rdkafka::{
    message::OwnedHeaders,
    producer::{future_producer::OwnedDeliveryResult, FutureProducer, FutureRecord},
    ClientConfig,
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
        .set(
            "bootstrap.servers",
            "kafka:9092",
        )
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    let mut record = FutureRecord::to("users")
        .key(id)
        .headers(OwnedHeaders::new().add("type", event.into()));

    if event != UserEvents::Deleted {
        record = record.payload(user);
    }

    producer.send(record, Duration::from_secs(0)).await
}
