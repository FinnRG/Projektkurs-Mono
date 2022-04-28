use kafka::VideoEvents;
use log::{error, info, warn};
use r2d2::PooledConnection;
use redis::{Client, Commands, RedisError};
use std::env;
use tonic::{transport::Server, Request, Response, Status};
use videos::v1::video_service_server::{VideoService, VideoServiceServer};
use videos::v1::Video;
use videos::v1::{
    CreateVideoRequest, CreateVideoResponse, GetVideoRequest, GetVideoResponse, UpdateVideoRequest,
    UpdateVideoResponse, Visibility,
};
#[macro_use]
extern crate lazy_static;

include!("../gen/mod.rs");

mod kafka;

lazy_static! {
    static ref JWTSECRET: String = env::var("JWTSECRET").unwrap();
    static ref POOL: r2d2::Pool<redis::Client> = {
        let client = redis::Client::open(env::var("REDIS_URL").unwrap()).unwrap();
        r2d2::Pool::<redis::Client>::builder()
            .max_size(15)
            .build(client)
            .unwrap()
    };
}

fn get_conn() -> Result<r2d2::PooledConnection<redis::Client>, Status> {
    match POOL.get() {
        Ok(conn) => Ok(conn),
        Err(e) => {
            error!("{:?}", e);
            Err(Status::internal("Failed to connect to Redis"))
        }
    }
}

fn get_video_from_redis(
    conn: &mut PooledConnection<Client>,
    id: &str,
) -> Result<Option<String>, RedisError> {
    conn.get::<_, Option<String>>(&id)
}

#[derive(Debug, Default)]
pub struct Videos {}

#[tonic::async_trait]
impl VideoService for Videos {
    async fn get_video(
        &self,
        request: Request<GetVideoRequest>,
    ) -> Result<Response<GetVideoResponse>, Status> {
        let id = request.into_inner().id;

        let mut conn = get_conn()?;

        return match get_video_from_redis(&mut conn, &id) {
            Ok(Some(video)) => {
                let parsed: Video = serde_json::from_str(video.as_ref()).unwrap();
                Ok(Response::new(GetVideoResponse {
                    video: Some(parsed),
                }))
            }
            Ok(None) => Err(Status::not_found(format!(
                "Couldn't find video with id: {}",
                id
            ))),
            Err(_) => Err(Status::internal("Internal Redis error")),
        };
    }

    async fn update_video(
        &self,
        request: Request<UpdateVideoRequest>,
    ) -> Result<Response<UpdateVideoResponse>, Status> {
        // Authorize request
        if rs_auth::user_id!(request).is_none() {
            return Err(Status::unauthenticated("User is not logged in"));
        }

        let mut conn = get_conn()?;
        let update_request = request.into_inner().video;

        // Checks that the video id is specified
        if update_request.is_none() || update_request.as_ref().unwrap().id.is_empty() {
            return Err(Status::invalid_argument("Video id not specified"));
        }

        let update_request = update_request.unwrap();
        let id = update_request.id;

        // Get current video from redis
        let video_str = match get_video_from_redis(&mut conn, &id) {
            Ok(r) => match r {
                Some(v) => v,
                None => return Err(Status::not_found("Video with id not found")),
            },
            Err(_) => return Err(Status::internal("Internal Redis error")),
        };

        // Update video with new values
        let mut video: Video =
            serde_json::from_str(&video_str).expect("Unable to parse stored json");
        if !update_request.title.is_empty() {
            video.title = update_request.title;
        }
        if !update_request.description.is_empty() {
            video.description = update_request.description;
        }

        let video_str = serde_json::to_string(&video).expect("Unable to stringify Video object");

        // Emit VideoChanged event
        if kafka::emit_video(&id, &video_str, VideoEvents::Changed)
            .await
            .is_err()
        {
            return Err(Status::internal("Internal kafka error"));
        }

        // Change video in redis
        if let Err(e) = conn.set::<_, _, ()>(&id, &video_str) {
            warn!(
                "Unable to update {} with {:?} because of {:?}",
                &id, &video_str, e
            );
        }

        Ok(Response::new(UpdateVideoResponse { video: Some(video) }))
    }

    async fn create_video(
        &self,
        request: Request<CreateVideoRequest>,
    ) -> Result<Response<CreateVideoResponse>, Status> {
        info!("New video creation request: {:?}", request);
        let author = rs_auth::user_id!(request);
        if author.is_none() {
            return Err(Status::unauthenticated("User is not logged in"));
        }

        let mut conn = get_conn()?;
        let create_request = request.into_inner();

        if create_request.title.is_empty() {
            return Err(Status::invalid_argument("Title can't be an empty string"));
        }

        let id = uuid::Uuid::new_v4();
        let video = serde_json::to_string(&Video {
            id: id.to_string(),
            title: create_request.title,
            description: create_request.description,
            author: author.unwrap(),
            date: chrono::offset::Local::now().to_string(),
            visibility: Visibility::Draft as i32,
        })
        .expect("Unable to stringify Video object");

        if kafka::emit_video(&id.to_string(), &video, VideoEvents::Created)
            .await
            .is_err()
        {
            return Err(Status::internal("Internal kafka error"));
        }

        if let Err(e) = conn.set::<_, _, ()>(id.to_string(), &video) {
            warn!(
                "Unable to create {} with {:?} because of {:?}",
                id.to_string(),
                &video,
                e
            );
        }

        Ok(Response::new(CreateVideoResponse { id: id.to_string() }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let _handle = tokio::spawn(kafka::receive_events());

    info!("Redis url: {}", env::var("REDIS_URL").unwrap());
    let addr = "0.0.0.0:8080".parse()?;
    let videos = Videos::default();

    info!("Starting VideoService");
    Server::builder()
        .add_service(VideoServiceServer::new(videos))
        .serve(addr)
        .await?;

    Ok(())
}
