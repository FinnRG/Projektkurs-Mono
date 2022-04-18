use auth_lib::{
    create_jwt,
};
use serde::Deserialize;
use tonic::{Request, Response, Status};
use tonic::transport::Server;
use videos::v1::Video;
use users::v1::*;
use users::v1::user_service_server::UserService;

include!("../gen/mod.rs");

#[derive(Debug, Default)]
pub struct Users {}

#[tonic::async_trait]
impl UserService for Users {
    async fn register(&self, request: Request<RegisterRequest>) -> Result<Response<RegisterResponse>, Status> {
        let request = request.into_inner();
        if request.name.is_empty() || request.email.is_empty() || request.password.is_empty() {
            return Err(Status::invalid_argument("name, email and password must be specified"))
        }


    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}