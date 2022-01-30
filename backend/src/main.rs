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
mod comment;
mod like;
mod redis;
mod tag;
mod upload;
mod user;
mod video;

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

#[database("postgres_logs")]
pub struct PostgresConn(diesel::PgConnection);

mod util {

    #[macro_export]
    macro_rules! cache {
        ($cache_helper: ident, $cache_fail: expr) => {{
            let cache_result = $cache_helper.get_cache(); 
            match cache_result {
                Ok(res) => res,
                _ => {
                    let res = $cache_fail;
                    $cache_helper.set_cache(&res);
                    res
                },
            }
        }};
    }
    pub(crate) use cache;

    #[macro_export]
    macro_rules! merge_params {
        ($pname: expr, $pval: expr) => {format!("{}={}", $pname, $pval)};
        ($pname: expr, $pvalue: expr, $($p2name: expr, $p2val: expr),*) => {
            format!("{}={}&{}", $pname, $pvalue, merge_params!($($p2name, $p2val),*))
        };
    }

    #[macro_export]
    macro_rules! invalidate {
        ($cache_helper: expr, $key: expr) => {
            let _: () = $cache_helper.del_cache($key);
        };
        ($cache_helper: expr, $base: expr, $( $pname: expr, $pval: expr),*) => {
            let key = format!("{}?{}", $base, merge_params!($($pname, $pval),*));
            invalidate!($cache_helper, key);
        };
    }
    pub(crate) use invalidate;

    #[macro_export]
    macro_rules! get_user_id {
        ($cookies: ident) => {
            match $cookies.get_private("user_id") {
                Some(id) => id.value().to_owned(),
                None => return Status::from_code(401).unwrap(),
            }
        };
    }
    pub(crate) use get_user_id;
}

#[get("/get/<name>")]
async fn get_file(mut name: String) -> ByteStream![Vec<u8>] {
    let bucket = get_bucket();
    let re = Regex::new(r"\d*\.(ts|m3u8)$").unwrap();
    let id = re.replace(&name, "").into_owned();

    // Reroutes the /get/name to /get/name.m3u8 request
    if !(name.ends_with(".ts") || name.ends_with(".m3u8")) {
        name += ".m3u8";
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
        .mount("/video", video::routes())
        .mount("/comment", comment::routes())
        .mount("/like", like::routes())
        .mount("/tag", tag::routes())
        .attach(cors)
        .attach(PostgresConn::fairing())
        .launch()
        .await?;

    Ok(())
}
