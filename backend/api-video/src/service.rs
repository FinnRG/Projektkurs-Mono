use crate::storage::Store;
use log::{info, warn};
use std::env;
use tonic::{transport::Server, Request, Response, Status};
use videos::v1::video_service_server::{VideoService, VideoServiceServer};
use videos::v1::{
    CreateVideoRequest, CreateVideoResponse, GetVideoRequest, GetVideoResponse, UpdateVideoRequest,
    UpdateVideoResponse, Video,
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

#[derive(Debug, Default)]
pub struct Videos {}

#[tonic::async_trait]
impl VideoService for Videos {
    async fn get_video(
        &self,
        request: Request<GetVideoRequest>,
    ) -> Result<Response<GetVideoResponse>, Status> {
        let id = request.into_inner().id;

        let mut store = Store::new();

        match store.get_video(&id) {
            Ok(video) => Ok(Response::new(GetVideoResponse { video: Some(video) })),
            Err(e) => Err(e.to_status()),
        }
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
