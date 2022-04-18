use log::{info, warn};
use r2d2::Pool;
use rdkafka::{
    config::RDKafkaLogLevel,
    consumer::{CommitMode, Consumer, DefaultConsumerContext, StreamConsumer},
    ClientConfig, Message,
};
use redis::Commands;
use std::env;
use tonic::{codegen::http::request, transport::Server, Request, Response, Status};
use videos::v1::video_service_server::{VideoService, VideoServiceServer};
use videos::v1::Video;
use videos::v1::{
    CreateVideoRequest, CreateVideoResponse, GetVideoRequest, GetVideoResponse, UpdateVideoRequest,
    UpdateVideoResponse,
};
#[macro_use]
extern crate rs_auth;
extern crate r2d2;
#[macro_use]
extern crate lazy_static;

include!("../gen/mod.rs");

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

// let context = DefaultConsumerContext::default();

// let consumer: StreamConsumer = ClientConfig::new()
//     .set("group.id", "video_api")
//     .set("bootstrap.servers", "localhost:9092")
//     .set("enable.partition.eof", "false")
//     .set("session.timeout.ms", "6000")
//     .set("enable.auto.commit", "true")
//     .set_log_level(RDKafkaLogLevel::Debug)
//     .create_with_context(context)
//     .expect("Consumer creation failed");

// consumer.subscribe(&vec!["upload"])
//     .expect("Can't subscribe to upload");

//     loop {
//         match consumer.recv().await {
//             Err(e) => warn!("Kafka error: {:?}", e),
//             Ok(m) => {
//                 let payload = match m.payload_view::<str>() {
//                     None => "",
//                     Some(Ok(s)) => s,
//                     Some(Err(e)) => { warn!("Error while deserializing payload: {:?}", e); "" },
//                 };
//                 info!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
//                   m.key(), payload, m.topic(), m.partition(), m.offset(), m.timestamp());
//                 consumer.commit_message(&m, CommitMode::Async).unwrap();
//             }
//         }
//     }

fn get_conn() -> Result<r2d2::PooledConnection<redis::Client>, Status> {
    match POOL.get() {
        Ok(conn) => Ok(conn),
        Err(e) => {
            log::error!("{:?}", e);
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
        request: Request<UpdateVideoRequest>,
    ) -> Result<Response<UpdateVideoResponse>, Status> {
        Err(Status::aborted("T"))
    }

    async fn create_video(
        &self,
        request: Request<CreateVideoRequest>,
    ) -> Result<Response<CreateVideoResponse>, Status> {
        request.metadata().get("auth").unwrap().as_ref();
        if !rs_auth::is_logged_in!(request) {
            return Err(Status::unauthenticated("User is not logged in"));
        }

        let mut conn = get_conn()?;

        Err(Status::aborted("T"))
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
