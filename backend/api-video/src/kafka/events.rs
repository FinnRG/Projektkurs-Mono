use strum::IntoStaticStr;
use crate::videos::v1::*;
use prost::Message;

#[derive(IntoStaticStr, PartialEq, Clone)]
pub enum VideoEvent {
    Created(VideoCreatedEvent),
    Deleted(VideoDeletedEvent),
    TitleChanged(VideoTitleChangedEvent),
    DescriptionChanged(VideoDescriptionChangedEvent),
    VisibilityChanged(VideoVisibilityChangedEvent),
    Uploaded(VideoUploadedEvent),
    Processed(VideoProcessedEvent),
    Finished(VideoFinishedEvent),
}

impl VideoEvent {
    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            VideoEvent::Created(e) => e.encode_to_vec(),
            VideoEvent::Deleted(e) => e.encode_to_vec(),
            VideoEvent::TitleChanged(e) => e.encode_to_vec(),
            VideoEvent::DescriptionChanged(e) => e.encode_to_vec(),
            VideoEvent::VisibilityChanged(e) => e.encode_to_vec(),
            VideoEvent::Uploaded(e) => e.encode_to_vec(),
            VideoEvent::Processed(e) => e.encode_to_vec(),
            VideoEvent::Finished(e) => e.encode_to_vec(),
        }
    }
}
