use actix_cors::Cors;
use actix_web::{post, web, App, HttpServer, Responder};
use auth_lib::Auth;
use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    util::Timeout,
    ClientConfig,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct UploadRequest {
    title: String,
    description: String,
}

#[derive(Serialize)]
struct InternalUpload {
    req: UploadRequest,
    id: Uuid,
    user_id: Uuid,
}

// TODO: Add auth
#[post("/upload")]
async fn upload(form: web::Json<UploadRequest>) -> impl Responder {
    let id = Uuid::new_v4();
    let req = InternalUpload {
        id,
        user_id: Uuid::new_v4(), // Auth!
        req: form.into_inner()
    };
    let event = serde_json::to_string(&req).unwrap();

    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("session.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    let delivery_status = producer.send(
        FutureRecord::<String, String>::to("upload").payload(&event),
        Timeout::Never,
    );
    println!("{:?}", delivery_status.await);
    "T"
}

#[actix::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().wrap(Cors::permissive()).service(upload))
        .bind(("0.0.0.0", 8000))?
        .run()
        .await
}
