use log::info;
use prost::Message as ProstMessage;
use rdkafka::message::BorrowedMessage;
use rdkafka::Message as KafkaMessage;

use super::VideoEvent;
use crate::kafka::emit_video_event;
use crate::storage::Store;
use crate::videos::v1::{Status as VideoStatus, Video, VideoCreatedEvent, VideoFinishedEvent};

pub async fn handle_event(t: &str, id: &str, m: &BorrowedMessage<'_>) {
    let store = Store::new();
    match t {
        "Deleted" => {
            store.del_video(id);
        }
        "Processed" => {
            handle_processed(store, id).await;
        }
        "Uploaded" | "Finished" => update_status(&store, id, t),
        "Created" => handle_created(store, id, m),
        "TitleChanged" | "DescriptionChanged" | "VisibilityChanged" => {
            // let payload = stringify_payload(m);
            // let video: Video = Video::from(payload);
            // store.set_video(&video);

            todo!()
        }
        _ => {}
    };
}

async fn handle_processed(store: Store, id: &str) {
    update_status(&store, id, "Finished");
    let event = VideoFinishedEvent { id: id.to_string() };
    info!(
        "Emitting finished event: {:?}",
        emit_video_event(id, VideoEvent::Finished(event)).await
    );
}

fn handle_created(store: Store, _id: &str, m: &BorrowedMessage<'_>) {
    if m.payload().is_none() {
        return;
    }
    let event = VideoCreatedEvent::decode(m.payload().unwrap()).expect("Error while decoding");
    let video = Video {
        id: event.id,
        title: event.title,
        description: event.description,
        author: event.author,
        date: event.date,
        visibility: event.visibility,
        status: VideoStatus::Draft as i32,
    };
    store.set_video(video);
}

fn update_status(store: &Store, id: &str, status: &str) {
    let mut video = Video::from(store.get_video(id));
    video.set_status(VideoStatus::from(status));
    store.set_video(video);
}
