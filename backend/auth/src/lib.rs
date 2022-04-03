use actix_web_httpauth::headers::authorization::Bearer;
use actix_web::http::header::Header;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web::{dev::ServiceRequest, HttpRequest};
use actix_web::Error;
use actix_web_httpauth::headers::authorization::Authorization;
use chrono::{Duration, Local};
use jsonwebtoken::{
    decode, encode, DecodingKey, EncodingKey, Header as JWTHeader, TokenData, Validation,
};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::env;

#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate diesel;

pub mod db;

lazy_static! {
    static ref JWT_SECRET: String = env::var("JWT_SECRET").unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
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

pub fn create_jwt(id: String) -> String {
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

pub fn get_jwt(auth: Bearer) -> Option<TokenData<Claims>> {
    let jwt = auth.token();
    decode(
        jwt,
        &DecodingKey::from_secret(JWT_SECRET.as_ref()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    )
    .ok()
}

// TODO: Reorganize this mess
pub fn parse_request(req: HttpRequest) -> Option<TokenData<Claims>> {
    let auth = Authorization::<Bearer>::parse(&req).ok()?;
    get_jwt(auth.into_scheme())
}

pub fn get_id(auth: Bearer) -> Option<String> {
    get_jwt(auth).map(|jwt| jwt.claims.sub)
}
