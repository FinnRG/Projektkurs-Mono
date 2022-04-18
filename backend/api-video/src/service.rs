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
    UpdateVideoResponse, Visibility,
};
extern crate r2d2;
extern crate rs_auth;
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
        let author = rs_auth::user_id!(request);
        if !author.is_some() {
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
        .unwrap();

        match conn.set::<_, _, ()>(id.to_string(), video) {
            Ok(_) => Ok(Response::new(CreateVideoResponse { id: id.to_string() })),
            Err(_) => Err(Status::internal("Internal Redis error")),
        }
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
