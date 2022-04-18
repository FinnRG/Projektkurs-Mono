use jsonwebtoken::{encode, EncodingKey, Header as JWTHeader, TokenData};
use lazy_static::lazy_static;
use rs_auth::Claims;

use std::env;

lazy_static! {
    static ref JWT_SECRET: String = env::var("JWT_SECRET").unwrap();
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
