use actix_web::dev::Payload;
use actix_web::http::header::Header;
use actix_web::{FromRequest, HttpRequest};
use actix_web_httpauth::headers::authorization::Authorization;
use actix_web_httpauth::headers::authorization::Bearer;
use chrono::{Duration, Local};
use jsonwebtoken::{
    decode, encode, DecodingKey, EncodingKey, Header as JWTHeader, TokenData, Validation,
};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::env;

#[cfg(feature = "build-binary")]
#[macro_use]
extern crate diesel_migrations;
#[cfg(feature = "build-binary")]
#[macro_use]
extern crate diesel;

#[cfg(feature = "build-binary")]
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

pub struct Auth(pub TokenData<Claims>);

impl Auth {
    pub fn from_request(req: &HttpRequest) -> Option<Self> {
        let auth = Authorization::<Bearer>::parse(req).ok()?;
        let scheme = auth.into_scheme();
        let decoded = decode(
            scheme.token(),
            &DecodingKey::from_secret(JWT_SECRET.as_ref()),
            &Validation::new(jsonwebtoken::Algorithm::HS256),
        )
        .ok()?;
        Some(Auth(decoded))
    }
}

impl FromRequest for Auth {
    type Error = actix_web::error::Error;
    type Future = futures::future::Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        match Auth::from_request(req) {
            Some(auth) => futures::future::ok(auth),
            None => futures::future::err(actix_web::error::ErrorBadRequest("No token supplied")),
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
