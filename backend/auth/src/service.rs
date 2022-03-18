use std::env;

use auth_lib::db::{check_password, create_user, run_migrations};
use auth_lib::rpc::{
    register_response::Type,
    auth_server::{Auth, AuthServer},
    LoginRequest, LoginResponse, RegisterRequest, RegisterResponse, ResponseType,
};
use chrono::{Duration, Local};
use jsonwebtoken::{encode, EncodingKey, Header};
use lazy_static::lazy_static;

use serde::{Deserialize, Serialize};
use tonic::{transport::Server, Request, Response, Status};

lazy_static! {
    static ref JWT_SECRET: String = env::var("JWT_SECRET").unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: i64,
}

fn get_exp() -> i64 {
    (Local::now() + Duration::days(1)).timestamp()
}

impl Claims {
    fn new(id: String) -> Self {
        Claims {
            sub: id,
            exp: get_exp(),
        }
    }
}

#[derive(Debug, Default)]
struct Authenticator {}

#[tonic::async_trait]
impl Auth for Authenticator {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let user = request.into_inner();
        let db_res = create_user(&user.name, &user.email, &user.password);

        Ok(Response::new(match db_res {
            Ok(id) => return_id(id),
            Err(err) => return_err(err),
        }))
    }

    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let cred = request.into_inner();
        match check_password(&cred.email, &cred.password) {
            Some(id) => {
                let claims = Claims::new(id);

                let jwt = match encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
                ) {
                    Ok(c) => c,
                    Err(_err) => panic!(),
                };

                Ok(Response::new(LoginResponse {
                    jwt,
                    result: ResponseType::Success as i32,
                }))
            }
            _ => Err(Status::aborted("T")),
        }
    }
}

fn return_id(id: String) -> RegisterResponse {
    let claims = Claims::new(id);

    let jwt = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    ) {
        Ok(c) => c,
        Err(_err) => panic!(),
    };

    RegisterResponse {
        jwt,
        result: Type::Success as i32,
    }
}

fn return_err(err: Type) -> RegisterResponse {
    RegisterResponse {
        jwt: String::from(""),
        result: err as i32,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::env_logger::init();
    run_migrations();

    let addr = "0.0.0.0:50051".parse()?;
    let authenticator = Authenticator::default();

    Server::builder()
        .add_service(AuthServer::new(authenticator))
        .serve(addr)
        .await?;

    Ok(())
}
