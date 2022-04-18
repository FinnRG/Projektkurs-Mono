use log::{info, warn};
use r2d2::Pool;
use rdkafka::{
    config::RDKafkaLogLevel,
    consumer::{CommitMode, Consumer, DefaultConsumerContext, StreamConsumer},
    ClientConfig, Message,
};

include!("../gen/mod.rs");

use tonic::{transport::Server, Request, Response, Status};
use videos::v1::video_service_server::{VideoService, VideoServiceServer};
use videos::v1::{
    CreateVideoRequest, CreateVideoResponse, GetVideoRequest, GetVideoResponse, UpdateVideoRequest,
    UpdateVideoResponse,
};

extern crate r2d2;
#[macro_use]
extern crate lazy_static;

lazy_static! {
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

#[derive(Debug, Default)]
pub struct Videos {}

#[tonic::async_trait]
impl VideoService for Videos {
    async fn get_video(
        &self,
        request: Request<GetVideoRequest>,
    ) -> Result<Response<GetVideoResponse>, Status> {
        Err(Status::aborted("T"))
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
