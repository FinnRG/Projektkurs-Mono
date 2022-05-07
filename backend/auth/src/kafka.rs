use crate::storage::Store;
use crate::users::v1::{
    UserEmailChangedEvent, UserNameChangedEvent, UserPasswordChanged, UserRegisteredEvent,
};
use crate::User;
use log::{info, warn};
use prost::Message as ProstMessage;
use rdkafka::consumer::Consumer;
use rdkafka::message::BorrowedMessage;
use rdkafka::{
    config::RDKafkaLogLevel,
    consumer::StreamConsumer,
    message::{Header, Headers, OwnedHeaders},
    producer::{future_producer::OwnedDeliveryResult, FutureProducer, FutureRecord},
    ClientConfig, Offset, Message
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
    let t = extract_event_type(msg);
    let key = extract_key(msg);

    if key.is_none() {
        return;
    }

    match (t, msg.payload()) {
        (Some("Deleted"), None) => {
            //TODO: Fix this
            store.del_user(key.unwrap());
        }
        // TODO: Handle other events
        (Some("Registered"), Some(p)) => {
            handle_registered(&mut store, p);
        }
        _ => {}
    };
}

fn handle_registered(store: &mut Store, payload: &[u8]) {
    let event = match UserRegisteredEvent::decode(payload) {
        Ok(e) => e,
        Err(_) => return,
    };

    let user = User {
        id: event.id,
        email: event.email,
        name: event.name,
        password: event.password,
    };

    store.set_user(&user);

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
