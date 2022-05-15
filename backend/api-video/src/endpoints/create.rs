use crate::storage::Store;
use crate::{
    kafka::{self, events::VideoEvent},
    videos::v1::{CreateVideoRequest, CreateVideoResponse, VideoCreatedEvent},
};
use tonic::{Response, Status};

pub async fn handle_create_request(
    req: CreateVideoRequest,
    author: &str,
) -> Result<Response<CreateVideoResponse>, Status> {
    if req.title.is_empty() {
        return Err(Status::invalid_argument("Title can't be an empty string"));
    }

    let id = uuid::Uuid::new_v4();

    let event = VideoCreatedEvent {
        id: id.to_string(),
        title: req.title,
        description: req.description,
        author: author.to_string(),
        date: chrono::offset::Local::now().to_string(),
        visibility: req.visibility as i32,
    };

    if kafka::emit_video_event(&id.to_string(), VideoEvent::Created(event))
        .await
        .is_err()
    {
        return Err(Status::internal("Internal kafka error"));
    }

    Ok(Response::new(CreateVideoResponse { id: id.to_string() }))
}
