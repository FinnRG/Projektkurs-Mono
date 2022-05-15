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
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

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
    static ref DATABASE_URL: String = env::var("DATABASE_URL").unwrap();
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

        let store = Store::new();

        let video = Video::from(store.get_video(&id));
        Ok(Response::new(GetVideoResponse { video: Some(video) }))
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

embed_migrations!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    init();

    let _handle = tokio::spawn(kafka::receive_events());

    let addr = "0.0.0.0:8080".parse()?;
    let videos = Videos::default();

    info!("Starting VideoService");
    Server::builder()
        .add_service(VideoServiceServer::new(videos))
        .serve(addr)
        .await?;

    Ok(())
}

fn init() {
    use diesel::prelude::*;
    embedded_migrations::run(
        &PgConnection::establish(&DATABASE_URL).expect("Unable to establish DB connection"),
    )
    .expect("Unable to run DB migrations");
}
