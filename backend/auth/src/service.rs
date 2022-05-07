use crate::{
    user::User,
    users::v1::{RegisterRequest, RegisterResponse},
};
use auth::create_jwt;
use log::info;
use std::env;
use tonic::transport::Server;
use tonic::{Request, Response, Status};
use users::v1::user_service_server::UserService;
use users::v1::user_service_server::UserServiceServer;
use users::v1::{LoginRequest, LoginResponse};

#[macro_use]
extern crate lazy_static;

include!("../gen/mod.rs");

mod kafka;
mod routes;
mod storage;
mod user;

lazy_static! {
    static ref JWTSECRET: String = env::var("JWTSECRET").unwrap();
    static ref POOL: r2d2::Pool<redis::Client> = {
        let client = redis::Client::open(env::var("REDIS_URL").unwrap()).unwrap();
        r2d2::Pool::<redis::Client>::builder()
            .max_size(15)
            .build(client)
            .unwrap()
    };
}

fn construct_response<'a, T>(resp: &'a mut Response<T>, id: &str) -> &'a Response<T> {
    resp.metadata_mut().insert(
        "authorization",
        create_jwt(id.to_string())
            .parse()
            .expect("Unable to convert id to Metadata value"),
    );
    resp
}

#[derive(Debug, Default)]
struct Users {}

#[tonic::async_trait]
impl UserService for Users {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        // Validate the request
        let request = request.into_inner();
        if request.name.is_empty() || request.email.is_empty() || request.password.is_empty() {
            return Err(Status::invalid_argument(
                "name, email and password must be specified",
            ));
        }
        return routes::register::handle_register_request(request).await;
    }
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let request = request.into_inner();
        return routes::login::handle_login_request(request).await;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let addr = "0.0.0.0:8080".parse()?;
    let users = Users::default();

    let _handle = tokio::spawn(kafka::receive_events());

    // let reflection = tonic_reflection::server::Builder::configure()
    //     .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
    //     .build()
    //     .expect("Unable to build reflection service using FILE_DESCRIPTOR_SET");

    info!("Serving gRPC on port: {}", "8080");
    Server::builder()
        // .add_service(reflection)
        .add_service(UserServiceServer::new(users))
        .serve(addr)
        .await?;
    Ok(())
}
