use argon2::{self, Config};
use auth::create_jwt;
use kafka::emit_user;
use log::error;
use log::info;
use log::trace;
use log::warn;
use rdkafka::message::ToBytes;
use redis::Commands;
use serde::{Deserialize, Serialize};
use std::env;
use std::thread;
use tonic::transport::Server;
use tonic::{Request, Response, Status};
use users::v1::user_service_server::UserService;
use users::v1::user_service_server::UserServiceServer;
use users::v1::FILE_DESCRIPTOR_SET;
use users::v1::{register_response::Result as RegisterResult, *};

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

#[derive(Serialize, Deserialize, Debug)]
struct User<'a> {
    id: &'a str,
    name: &'a str,
    email: &'a str,
    password: &'a str,
}

#[derive(Debug, Default)]
struct Users {}

#[tonic::async_trait]
impl UserService for Users {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        info!("New register request: {:?}", request);
        // Validate the request
        let request = request.into_inner();
        if request.name.is_empty() || request.email.is_empty() || request.password.is_empty() {
            return Err(Status::invalid_argument(
                "name, email and password must be specified",
            ));
        }
        info!("Register request validated");

        let mut conn = get_conn()?;
        let id = uuid::Uuid::new_v4();
        trace!("Id generated: {}", id);

        // Generate the hash using Argon2 and a random salt using openssl
        let mut buf = [0u8; 128];
        openssl::rand::rand_bytes(&mut buf).expect("Unable to generate random salt");
        let config = Config::default();
        let hash = argon2::hash_encoded(request.password.to_bytes(), &buf, &config)
            .expect("Unable to hash password");

        // Stringify the user struct
        let user = serde_json::to_string(&User {
            id: &id.to_string(),
            name: &request.name,
            email: &request.email,
            password: &hash,
        })
        .expect("Unable to stringify the user struct");
        trace!("User struct created: {:?}", user);

        // Emit UserRegistered event
        if emit_user(&id.to_string(), &user, kafka::UserEvents::Registered)
            .await
            .is_err()
        {
            warn!("Failed to emit UserRegistered event");
            return Err(Status::internal("Internal kafka error"));
        }

        match conn.get::<_, Option<String>>(&request.email) {
            Ok(Some(_)) => Err(Status::already_exists("Email duplicate")),
            Err(_) => {
                warn!("Failed to connect to redis");
                Err(Status::internal("Failed to connect to redis"))
            }
            _ => {
                if let Err(e) = conn.set::<_, _, ()>(id.to_string(), &user) {
                    warn!("Unable to set {} to {} because of {:?}", id, &user, e);
                }

                if let Err(e) = conn.set::<_, _, ()>(&request.email, &hash) {
                    warn!("Unable to set {} to {} because of {:?}", id, &user, e);
                }

                // Construct response with added metadata field
                let mut response = Response::new(RegisterResponse {
                    res: RegisterResult::Accepted as i32,
                });
                response.metadata_mut().insert(
                    "authorization",
                    create_jwt(id.to_string())
                        .parse()
                        .expect("Unable to convert id to Metadata value"),
                );

                info!("Register request successful");
                return Ok(response);
            }
        }
    }
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        info!("New register request: {:?}", request);
        let request = request.into_inner();
        let mut conn = get_conn()?;
        
        return match conn.get::<_, Option<String>>(&request.email) {
            Err(_) => Err(Status::internal("Internal Redis error")),
            Ok(None) => Err(Status::invalid_argument("Couldn't find email address")),
            Ok(Some(hash)) => {
                match argon2::verify_encoded(&hash, request.password.as_bytes()) {
                    Ok(true) => Ok(Response::new(LoginResponse {})),
                    Ok(false) => Err(Status::invalid_argument("Wrong password")),
                    Err(_) => Err(Status::internal("Internal hashing error")),
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let addr = "0.0.0.0:8080".parse()?;
    let users = Users::default();

    thread::spawn(kafka::receive_events);

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
