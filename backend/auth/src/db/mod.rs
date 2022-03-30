pub mod models;
pub mod schema;

use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};
use models::{NewUser, User};
use schema::users;
use serde::Serialize;
use std::env;
use uuid::Uuid;

embed_migrations!();

pub fn run_migrations() {
    let connection = establish_connection();

    embedded_migrations::run(&connection);
}

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Can't establish connection to {}", database_url))
}

pub enum CreateUserError {
    UniqueViolation,
    Other,
}

pub fn create_user(name: &str, email: &str, password: &str) -> Result<String, CreateUserError> {
    let hash = hash(&password, DEFAULT_COST).expect("Unable to hash");
    let id = Uuid::new_v4().to_string();

    let new_user = NewUser {
        id: &id,
        name,
        email,
        password: &hash,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&establish_connection())
        .map_err(|err| match err {
            Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                CreateUserError::UniqueViolation
            }
            _ => CreateUserError::Other,
        })
        .map(|_| id.to_string())
}

pub fn check_password(email: &str, password: &str) -> Option<String> {
    match users::table
        .filter(users::email.eq(email))
        .get_result::<User>(&establish_connection())
    {
        Ok(user) => match verify(password, &user.password) {
            Ok(_) => Some(user.id),
            Err(_) => None,
        },
        Err(_) => None,
    }
}
