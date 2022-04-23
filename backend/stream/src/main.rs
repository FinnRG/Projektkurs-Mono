extern crate dotenv;

use std::env;

use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use s3::{creds::Credentials, Bucket, Region};

struct Storage {
    region: Region,
    credentials: Credentials,
    bucket: String,
}

fn get_storage() -> Storage {
    let endpoint = env::var("MINIO_URL").expect("MINIO_URL must be set.");
    Storage {
        region: Region::Custom {
            region: "minio".into(),
            endpoint,
        },
        credentials: Credentials {
            access_key: Some("admin".to_owned()),
            secret_key: Some("strongPassword".to_owned()),
            security_token: None,
            session_token: None,
        },
        bucket: "videos".to_string(),
    }
}

fn get_bucket() -> Bucket {
    let minio = get_storage();
    Bucket::new_with_path_style(&minio.bucket, minio.region, minio.credentials).unwrap()
}

#[get("/stream/get/{id}/{name}")]
async fn stream(path: web::Path<(String, Option<String>)>) -> impl Responder {
    let (id, name) = path.into_inner();
    let bucket = get_bucket();

    let name = name.unwrap_or("hls.m3u8".to_string());

    let path = format!("{}/{}", id, name);
    let resp = bucket.get_object(path).await;
    match resp {
        Ok((data, _)) => HttpResponse::Ok().body(data),
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::NotFound().finish()
        }
    }
}

// TODO: Add logging
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method()
            .supports_credentials()
            .max_age(3600);

        App::new().wrap(cors).service(stream)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
