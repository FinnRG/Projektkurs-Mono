use crate::users::v1::{
    UserEmailChangedEvent, UserNameChangedEvent, UserPasswordChanged, UserRegisteredEvent,
};
use prost::Message as ProstMessage;
use rdkafka::{
    message::{Header, OwnedHeaders},
    producer::{future_producer::OwnedDeliveryResult, FutureProducer, FutureRecord},
    ClientConfig,
};
use std::time::Duration;
use strum::IntoStaticStr;

#[derive(IntoStaticStr, PartialEq, Clone)]
pub enum UserEvent {
    Registered(UserRegisteredEvent),
    EmailChanged(UserEmailChangedEvent),
    PasswordChanged(UserPasswordChanged),
    NameChanged(UserNameChangedEvent),
    Deleted,
}

impl UserEvent {
    fn as_bytes(&self) -> Option<Vec<u8>> {
        match self {
            UserEvent::Registered(e) => Some(e.encode_to_vec()),
            UserEvent::EmailChanged(e) => Some(e.encode_to_vec()),
            UserEvent::NameChanged(e) => Some(e.encode_to_vec()),
            UserEvent::PasswordChanged(e) => Some(e.encode_to_vec()),
            UserEvent::Deleted => None,
        }
    }
}

pub async fn emit_user(id: &str, event: &UserEvent) -> OwnedDeliveryResult {
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

    let bytes = event.as_bytes();

    if bytes.is_some() {
        record = record.payload(bytes.as_ref().unwrap());
    }

    let res = producer.send(record, Duration::from_secs(0)).await;
    res
}
