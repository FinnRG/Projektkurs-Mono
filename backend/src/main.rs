#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use regex::Regex;
use rocket::http::Method;
use rocket::response::stream::ByteStream;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use rocket_sync_db_pools::{database, diesel};
use s3::{creds::Credentials, Bucket, BucketConfiguration, Region};
use std::env;
use std::{error::Error, process::Command};
use tokio::fs;
use uuid::Uuid;

// Import routes separated into different files
mod upload;
mod user;

extern crate s3;

const CHUNK_SIZE: u64 = 1024 * 1024;

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
            endpoint: endpoint,
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

#[database("postgres_logs")]
struct PostgresConn(diesel::PgConnection);

#[get("/get/<name>")]
async fn get_file(mut name: String) -> ByteStream![Vec<u8>] {
    let bucket = get_bucket();
    let re = Regex::new(r"\d*\.(ts|m3u8)$").unwrap();
    let id = re.replace(&name, "").into_owned();

    // Reroutes the /get/name to /get/name.m3u8 request
    if !(name.ends_with(".ts") || name.ends_with(".m3u8")) {
        name = name + ".m3u8";
    }

    let path = format!("media/{}/output/{}", id, name);

    ByteStream! {
        let mut i = 0;
        loop {
            match bucket.get_object_range(path.clone(), i, Some(i + CHUNK_SIZE)).await {
                Ok((data, 200..=299)) => yield data,
                _ => break,
            }
            i += CHUNK_SIZE + 1;
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let bucket = get_bucket();
    Bucket::create_with_path_style(
        &bucket.name,
        bucket.region,
        bucket.credentials,
        BucketConfiguration::default(),
    )
    .await?;

    let allowed_origins = AllowedOrigins::all();

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    rocket::build()
        .mount("/", routes![get_file])
        .mount("/upload", upload::routes())
        .mount("/user", user::routes())
        .attach(cors)
        .attach(PostgresConn::fairing())
        .launch()
        .await?;

    Ok(())
}
