use crate::storage::{Store, StoreError};
use crate::{
    kafka::{self, VideoEvents},
    videos::v1::{UpdateVideoRequest, UpdateVideoResponse, Video, Visibility},
};
use tonic::{Response, Status};

pub async fn handle_update_request(
    req: UpdateVideoRequest,
) -> Result<Response<UpdateVideoResponse>, Status> {
    let mut store = Store::new();
    let changed_video = req.video;

    // Checks that the video id is specified
    if video_id_present(&changed_video) {
        return Err(Status::invalid_argument("Video id not specified"));
    }

    let changed_video = changed_video.unwrap();
    let id = &changed_video.id;

    // Get current video from redis
    let mut curr_video = match store.get_video(id) {
        Ok(video) => video,
        Err(StoreError::NotFound) => return Err(Status::not_found("Video with id not found")),
        Err(StoreError::Internal(_)) => return Err(Status::internal("Internal Redis error")),
    };

    if let Err(e) = handle_changed_video(&mut store, &mut curr_video, changed_video).await {
        return Err(e);
    }

    Ok(Response::new(UpdateVideoResponse {
        video: Some(curr_video),
    }))
}

// Checks if the video has an id attribute
fn video_id_present(video: &Option<Video>) -> bool {
    video.is_none() || video.as_ref().unwrap().id.is_empty()
}

// Emits new events based on the changes to the video. Doesn't check if the curr_video and changed_video are correct
async fn handle_changed_video(
    store: &mut Store,
    curr_video: &mut Video,
    changed_video: Video,
) -> Result<(), Status> {
    let events = update_video(curr_video, changed_video);

    let video_str = curr_video.to_json();

    for event in events {
        if kafka::emit_video_event(&curr_video.id, &video_str, event)
            .await
            .is_err()
        {
            return Err(Status::internal("Internal kafka error"));
        }
    }

    store.set_video(curr_video);

    Ok(())
}

// Updates the current video with new values from the changed video and returns a list of events that should be published
fn update_video(curr_video: &mut Video, changed_video: Video) -> Vec<VideoEvents> {
    let mut events: Vec<VideoEvents> = vec![];

    if !changed_video.title.is_empty() {
        curr_video.title = changed_video.title;
        events.push(VideoEvents::TitleChanged);
    }

    if !changed_video.description.is_empty() {
        curr_video.description = changed_video.description;
        events.push(VideoEvents::DescriptionChanged);
    }

    if changed_video.visibility != Visibility::Unspecified as i32 {
        curr_video.visibility = changed_video.visibility;
        events.push(VideoEvents::VisibilityChanged);
    }

    events
}
