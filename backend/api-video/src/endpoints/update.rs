use crate::storage::Store;
use crate::{
    kafka::{self, events::VideoEvent},
    videos::v1::{
        UpdateVideoRequest, UpdateVideoResponse, Video, VideoDescriptionChangedEvent,
        VideoTitleChangedEvent, VideoVisibilityChangedEvent, Visibility,
    },
};
use tonic::{Response, Status};

pub async fn handle_update_request(
    req: UpdateVideoRequest,
) -> Result<Response<UpdateVideoResponse>, Status> {
    let changed_video = req.video;

    // Checks that the video id is specified
    if video_id_present(&changed_video) {
        return Err(Status::invalid_argument("Video id not specified"));
    }

    let changed_video = changed_video.unwrap();

    // Get current video from database
    let mut store = Store::new();
    let curr_video = match update_video(&mut store, changed_video).await {
        Ok(v) => v,
        Err(status) => return Err(status),
    };

    Ok(Response::new(UpdateVideoResponse {
        video: Some(curr_video),
    }))
}

// Checks if the video has an id attribute
fn video_id_present(video: &Option<Video>) -> bool {
    video.is_none() || video.as_ref().unwrap().id.is_empty()
}

async fn update_video(store: &mut Store, changed_video: Video) -> Result<Video, Status> {
    let id = &changed_video.id;
    let mut curr_video = Video::from(store.get_video(id));

    if let Err(e) = handle_changed_video(store, &mut curr_video, changed_video).await {
        return Err(e);
    }
    Ok(curr_video)
}

// Emits new events based on the changes to the video. Doesn't check if the curr_video and changed_video are correct
async fn handle_changed_video(
    store: &mut Store,
    curr_video: &mut Video,
    changed_video: Video,
) -> Result<(), Status> {
    let events = update_video_object(curr_video, changed_video);

    for event in events {
        if kafka::emit_video_event(&curr_video.id, event)
            .await
            .is_err()
        {
            return Err(Status::internal("Internal kafka error"));
        }
    }

    store.set_video(curr_video.clone());

    Ok(())
}

// Updates the current video with new values from the changed video and returns a list of events that should be published
fn update_video_object(curr_video: &mut Video, changed_video: Video) -> Vec<VideoEvent> {
    let mut events: Vec<VideoEvent> = vec![];

    if !changed_video.title.is_empty() {
        curr_video.title = changed_video.title;
        let event = VideoTitleChangedEvent {
            title: curr_video.title.clone(),
        };
        events.push(VideoEvent::TitleChanged(event));
    }

    if !changed_video.description.is_empty() {
        curr_video.description = changed_video.description;
        let event = VideoDescriptionChangedEvent {
            description: curr_video.description.clone(),
        };
        events.push(VideoEvent::DescriptionChanged(event));
    }

    if changed_video.visibility != Visibility::Unspecified as i32 {
        curr_video.visibility = changed_video.visibility;
        let event = VideoVisibilityChangedEvent {
            visibility: curr_video.visibility,
        };
        events.push(VideoEvent::VisibilityChanged(event));
    }

    events
}
