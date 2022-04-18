use auth_lib::{
    create_jwt,
};
use serde::Deserialize;
use tonic::transport::Server;
use videos::v1::Video;

include!("../gen/mod.rs");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}