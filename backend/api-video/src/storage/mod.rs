pub mod models;
pub mod schema;

use crate::storage::models::DBVideo;
use crate::videos::v1::Video;
use crate::DATABASE_URL;
use diesel::prelude::*;
use diesel::PgConnection;
use diesel::pg::upsert::*;

use redis::{ErrorKind, RedisError};
use tonic::Status;

pub struct Store {
    conn: PgConnection,
}

#[derive(Debug)]
pub enum StoreError {
    NotFound,
    Internal(RedisError),
}

impl StoreError {
    fn client_error() -> StoreError {
        StoreError::Internal(RedisError::from((ErrorKind::ClientError, "Clienterror")))
    }

    pub fn to_status(&self) -> Status {
        match self {
            StoreError::Internal(_) => Status::internal("Internal redis error"),
            StoreError::NotFound => Status::not_found("Ressource not found"),
        }
    }
}

impl Store {
    pub fn new() -> Self {
        let conn = PgConnection::establish(&DATABASE_URL).expect("Unable to connect to Postgres");
        Store { conn }
    }

    pub fn set_video(&self, v: Video) {
        use schema::videos;

        diesel::insert_into(videos::table)
            .values(models::DBVideo::from(v))
            .on_conflict(on_constraint("videos_pkey"))
            .do_nothing()
            .execute(&self.conn)
            .expect("Failed to set new video");
    }

    pub fn get_video(&self, id: &str) -> DBVideo {
        use schema::videos;

        videos::table
            .find(id)
            .first::<DBVideo>(&self.conn)
            .expect("Error loading post")
    }

    pub fn del_video(&self, id: &str) {
        use schema::videos;

        diesel::delete(videos::table.find(id))
            .execute(&self.conn)
            .expect("Error deleting posts");
    }
}
