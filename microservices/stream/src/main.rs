use std::env;

use actix_cors::Cors;
use actix_web::{get, post, Responder, HttpResponse, HttpServer, App, web};
use regex::Regex;
use s3::{creds::Credentials, Bucket, BucketConfiguration, Region};

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
            access_key: Some("minio-admin".to_owned()),
            secret_key: Some("strongPassword".to_owned()),
            security_token: None,
            session_token: None,
        },
        bucket: "media".to_string(),
    }
}

fn get_bucket() -> Bucket {
    let minio = get_storage();
    Bucket::new_with_path_style(&minio.bucket, minio.region, minio.credentials).unwrap()
}

#[get("/get/{name}")]
async fn stream(name: web::Path<String>) -> impl Responder {
    let mut name = name.into_inner();
    let bucket = get_bucket();
    let re = Regex::new(r"\d*\.(ts|m3u8)$").unwrap();
    let id = re.replace(&name, "").into_owned();

    // Reroutes the /get/name to /get/name.m3u8 request
    if !(name.ends_with(".ts") || name.ends_with(".m3u8")) {
        name += ".m3u8";
    }

    let path = format!("media/{}/output/{}", id, name);
    let resp = bucket.get_object(path).await;
    match resp {
        Ok((data, _)) => HttpResponse::Ok().body(data),
        _ => HttpResponse::NotFound().finish()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    HttpServer::new(|| {

        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(stream)
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
