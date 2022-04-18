use log::error;
use redis::Commands;
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
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
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

        return match conn.get::<_, Option<String>>(&id) {
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
        _request: Request<UpdateVideoRequest>,
    ) -> Result<Response<UpdateVideoResponse>, Status> {
        Err(Status::aborted("T"))
    }

    async fn create_video(
        &self,
        request: Request<CreateVideoRequest>,
    ) -> Result<Response<CreateVideoResponse>, Status> {
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

        if conn.set::<_, _, ()>(id.to_string(), &video).is_err() {
            return Err(Status::internal("Internal Redis error"));
        }

        if kafka::emit_video_created(&video, &id.to_string())
            .await
            .is_err()
        {
            return Err(Status::internal("Internal kafka error"));
        }

        Ok(Response::new(CreateVideoResponse { id: id.to_string() }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:8080".parse()?;
    let videos = Videos::default();

    Server::builder()
        .add_service(VideoServiceServer::new(videos))
        .serve(addr)
        .await?;

    Ok(())
}
