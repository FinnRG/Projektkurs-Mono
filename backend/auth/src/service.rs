use std::{env};

use actix_cors::Cors;
use actix_web::{
    error::ParseError,
    get,
    http::header::Header,
    post,
    web::{Either, Form, Json},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_web_httpauth::{
    headers::authorization::{Authorization, Bearer},
};
use auth_lib::{
    db::{check_password, create_user, CreateUserError},
};
use chrono::{Duration, Local};
use jsonwebtoken::{encode, EncodingKey, Header as JWTHeader};
use lazy_static::lazy_static;
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

#[derive(Deserialize)]
struct Register {
    name: String,
    email: String,
    password: String,
}

fn create_jwt(id: String) -> String {
    let claims = Claims::new(id);

    match encode(
        &JWTHeader::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    ) {
        Ok(c) => c,
        Err(_err) => panic!(),
    }
}

fn create_auth_response(id: String) -> HttpResponse {
    let jwt = create_jwt(id);
    let credentials = Bearer::new(jwt);
    HttpResponse::Ok()
        .insert_header(("Authorization", credentials.to_string()))
        .finish()
}

#[post("/register")]
async fn register(form: Either<Json<Register>, Form<Register>>) -> impl Responder {
    let Register {
        name,
        email,
        password,
    } = form.into_inner();
    let db_res = create_user(&name, &email, &password);

    match db_res {
        Ok(id) => create_auth_response(id),
        Err(e) => match e {
            CreateUserError::UniqueViolation => HttpResponse::Conflict().finish(),
            _ => HttpResponse::ServiceUnavailable().finish(),
        },
    }
}

#[post("/login")]
async fn login(form: Either<Json<Register>, Form<Register>>) -> impl Responder {
    let cred = form.into_inner();

    match check_password(&cred.email, &cred.password) {
        Some(id) => create_auth_response(id),
        _ => HttpResponse::Unauthorized().finish(),
    }
}

#[get("/id")]
async fn get_id(req: HttpRequest) -> Result<String, ParseError> {
    let auth = Authorization::<Bearer>::parse(&req)?;
    Ok(auth.as_ref().token().to_string())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(register)
            .service(get_id)
            .wrap(Cors::permissive())
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
