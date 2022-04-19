use argon2::{self, Config};
use auth_lib::create_jwt;
use kafka::emit_user;
use log::error;
use log::warn;
use rdkafka::message::ToBytes;
use redis::Commands;
use serde::{Deserialize, Serialize};
use std::env;
use tonic::transport::Server;
use tonic::{Request, Response, Status};
use users::v1::user_service_server::UserService;
use users::v1::{register_response::Result as RegisterResult, *};
use users::v1::user_service_server::UserServiceServer;

#[macro_use]
extern crate lazy_static;

include!("../gen/mod.rs");

mod kafka;

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

fn get_conn() -> Result<r2d2::PooledConnection<redis::Client>, Status> {
    match POOL.get() {
        Ok(conn) => Ok(conn),
        Err(e) => {
            error!("{:?}", e);
            Err(Status::internal("Failed to connect to Redis"))
        }
    }
}

#[derive(Serialize, Deserialize)]
struct User {
    id: String,
    name: String,
    email: String,
    password: String,
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

        let mut conn = get_conn()?;
        let id = uuid::Uuid::new_v4();

        // Generate the hash using Argon2 and a random salt using openssl
        let mut buf = [0u8; 128];
        openssl::rand::rand_bytes(&mut buf).expect("Unable to generate random salt");
        let config = Config::default();
        let hash = argon2::hash_encoded(request.password.to_bytes(), &buf, &config)
            .expect("Unable to hash password");

        // Stringify the user struct
        let user = serde_json::to_string(&User {
            id: id.to_string(),
            name: request.name,
            email: request.email,
            password: hash,
        })
        .expect("Unable to stringify the user struct");

        // Emit UserRegistered event
        if emit_user(&id.to_string(), &user, kafka::UserEvents::Registered)
            .await
            .is_err()
        {
            return Err(Status::internal("Internal kafka error"));
        }

        if let Err(e) = conn.set::<_, _, ()>(id.to_string(), &user) {
            warn!("Unable to set {} to {} because of {:?}", id, &user, e);
        }

        // Construct response with added metadata field
        let mut response = Response::new(RegisterResponse {
            res: RegisterResult::Accepted as i32,
        });
        response.metadata_mut().insert(
            "auth",
            create_jwt(id.to_string())
                .parse()
                .expect("Unable to convert id to Metadata value"),
        );

        return Ok(response);
    }
    async fn login(&self, _request: Request<LoginRequest>) -> Result<Response<LoginResponse>, Status> {
        Err(Status::internal("Unimplemented"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:8080".parse()?;
    let users = Users::default();

    Server::builder()
        .add_service(UserServiceServer::new(users))
        .serve(addr)
        .await?;
    Ok(())
}
