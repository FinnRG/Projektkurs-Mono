use crate::POOL;
use crate::{videos::v1::Video, SCYLLA_URL};
use log::{error, warn};
use r2d2::PooledConnection;
use redis::{Client, Commands, ErrorKind, RedisError};
use scylla::{Session, SessionBuilder};
use tonic::Status;

pub struct Store {
    session: Session,
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
    pub async fn new() -> Self {
        let session = SessionBuilder::new()
            .known_node(SCYLLA_URL.as_str())
            .build()
            .await
            .unwrap();
        session.query("CREATE KEYSPACE IF NOT EXISTS ks WITH REPLICATION = {'class' : 'SimpleStrategy', 'replication_factor' : 1}", &[]).await.expect("Unable to create keyspace");

        session.query("CREATE TABLE IF NOT EXISTS ks.videos (id uuid primary key, title text, description text, author uuid, date timestamp, visibility int, status int)", &[]).await.expect("Unable to create database");

        Store { session }
    }

    pub fn set_video(&mut self, video: &Video) -> Result<(), StoreError> {
        todo!()
        // let video_str = &video.to_json();
        // let id = &video.id;

        // if let Err(e) = conn.set::<_, _, ()>(id, video_str) {
        //     warn!(
        //         "Unable to update {} with {:?} because of {:?}",
        //         id, video_str, e
        //     );
        // }
    }

    pub fn get_video(&mut self, id: &str) -> Result<Video, StoreError> {
        // if self.conn.is_none() {
        //     return Err(StoreError::client_error());
        // }
        // let conn = self.conn.as_mut().unwrap();

        // match conn.get::<_, Option<String>>(&id) {
        //     Ok(Some(v)) => Ok(Video::from(v)),
        //     Ok(None) => Err(StoreError::NotFound),
        //     Err(e) => Err(StoreError::Internal(e)),
        // }
        todo!()
    }

    pub fn del_video(&mut self, id: &str) -> Result<(), StoreError> {
        // if self.conn.is_none() {
        //     return Err(StoreError::client_error());
        // }
        // let conn = self.conn.as_mut().unwrap();
        // if let Err(e) = conn.del::<_, ()>(id) {
        //     warn!("Unable to delete {:?} because of {}", id, e);
        // }

        todo!()
    }
}
