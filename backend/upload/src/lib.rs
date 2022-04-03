use s3::{creds::Credentials, Bucket, BucketConfiguration, Region};
use std::env;

extern crate s3;
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
            access_key: Some(env::var("MINIO_USERNAME").expect("MINIO_USERNAME must be set.")),
            secret_key: Some(env::var("MINIO_PASSWORD").expect("MINIO_PASSWORD must be set.")),
            security_token: None,
            session_token: None,
        },
        bucket: env::var("MINIO_BUCKET").unwrap_or("media".into()),
    }
}

fn get_bucket() -> Bucket {
    let minio = get_storage();
    Bucket::new_with_path_style(&minio.bucket, minio.region, minio.credentials).unwrap()
}
