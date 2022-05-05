use log::{error, info, warn};
use r2d2::PooledConnection;
use redis::{Client, Commands, RedisError};
use std::env;
use tonic::{transport::Server, Request, Response, Status};
use videos::v1::video_service_server::{VideoService, VideoServiceServer};
use videos::v1::{
    CreateVideoRequest, CreateVideoResponse, GetVideoRequest, GetVideoResponse,
    UpdateVideoRequest, UpdateVideoResponse, Video,
};
#[macro_use]
extern crate lazy_static;

include!("../gen/mod.rs");

mod endpoints;
mod kafka;
mod storage;
mod video;

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

#[deprecated]
fn get_conn() -> Result<r2d2::PooledConnection<redis::Client>, Status> {
    match POOL.get() {
        Ok(conn) => Ok(conn),
        Err(e) => {
            error!("{:?}", e);
            Err(Status::internal("Failed to connect to Redis"))
        }
    }
}

#[deprecated]
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
        endpoints::update::handle_update_request(request.into_inner()).await
    }

    async fn create_video(
        &self,
        request: Request<CreateVideoRequest>,
    ) -> Result<Response<CreateVideoResponse>, Status> {
        let author = rs_auth::user_id!(request);
        if author.is_none() {
            return Err(Status::unauthenticated("User is not logged in"));
        }

        endpoints::create::handle_create_request(request.into_inner(), &author.unwrap()).await
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
