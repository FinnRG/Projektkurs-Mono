use std::{env, str::Bytes};

use actix_cors::Cors;
use actix_web::{
    dev::ServiceRequest,
    post,
    web::{get, post, Either, Form, Json},
    App, Error, HttpServer, Responder,
};
use auth_lib::db::create_user;
use chrono::{Duration, Local};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use lazy_static::lazy_static;
use log::{error, trace};
use msostream::establish_connection;
use msostream::user::check_password;
use serde::{Deserialize, Serialize};

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

#[tokio::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::env_logger::init();
    msostream::run_db_migrations();

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
