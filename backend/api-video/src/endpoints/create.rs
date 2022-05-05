use crate::storage::Store;
use crate::{
    kafka::{self, VideoEvents},
    videos::v1::{CreateVideoRequest, CreateVideoResponse, Status as VideoStatus, Video},
};
use tonic::{Response, Status};

pub async fn handle_create_request(
    req: CreateVideoRequest,
    author: &str,
) -> Result<Response<CreateVideoResponse>, Status> {
    let mut store = Store::new();

    if req.title.is_empty() {
        return Err(Status::invalid_argument("Title can't be an empty string"));
    }

    let id = uuid::Uuid::new_v4();
    let video = &Video {
        id: id.to_string(),
        title: req.title,
        description: req.description,
        author: author.to_string(),
        date: chrono::offset::Local::now().to_string(),
        visibility: req.visibility as i32,
        status: VideoStatus::Draft as i32,
    };

    let video_str = serde_json::to_string(video).expect("Unable to deserialize video");

    if kafka::emit_video_event(&id.to_string(), &video_str, VideoEvents::Created)
        .await
        .is_err()
    {
        return Err(Status::internal("Internal kafka error"));
    }

    store.set_video(&video);

    Ok(Response::new(CreateVideoResponse { id: id.to_string() }))
}
