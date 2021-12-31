pub mod models;
pub mod schema;

pub mod comment;
pub mod user;
pub mod video;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE URL must be set!");
    PgConnection::establish(&database_url).expect(&format!("Error connection to {}", database_url))
}
