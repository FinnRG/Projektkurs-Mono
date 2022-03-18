#[macro_use]
extern crate diesel_migrations;

pub mod rpc {
    tonic::include_proto!("auth");
}

pub mod db {

    use diesel::pg::PgConnection;
    use diesel::prelude::*;
    use std::env;

    embed_migrations!();

    pub fn establish_connection() -> PgConnection {
        let database_url = env::var("DATABASE_URL").expect("DATABASE URL must be set");
        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Can't establish connection to {}", database_url))
    }

    pub fn create_user(name: &str, email: &str, password: &str) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        msostream::user::create_user(&establish_connection(), &id, name, email, password);
        id
    }

    pub fn check_password(email: &str, password: &str) -> Option<String> {
        msostream::user::check_password(&establish_connection(), email, password)
    }

    pub fn run_migrations() {
        let connection = establish_connection();

        embedded_migrations::run(&connection);
    }
}
