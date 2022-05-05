use crate::videos::v1::Video;
use crate::POOL;
use log::{error, warn};
use r2d2::PooledConnection;
use redis::{Client, Commands, ErrorKind, RedisError};
use tonic::Status;

pub struct Store {
    conn: Option<PooledConnection<Client>>,
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

    pub fn to_status(self) -> Status {
        match self {
            StoreError::Internal(_) => Status::internal("Internal redis error"),
            StoreError::NotFound => Status::not_found("Ressource not found"),
        }
    }
}

impl Store {
    pub fn new() -> Self {
        let conn = POOL.get();

        if conn.is_err() {
            error!("Unable to establish redis connection");
            return Store { conn: None };
        };

        Store {
            conn: Some(conn.unwrap()),
        }
    }

    pub fn set_video(&mut self, video: &Video) -> Result<(), StoreError> {
        if self.conn.is_none() {
            return Err(StoreError::client_error());
        }
        let conn = self.conn.as_mut().unwrap();

        let video_str = &video.to_json();
        let id = &video.id;

        if let Err(e) = conn.set::<_, _, ()>(id, video_str) {
            warn!(
                "Unable to update {} with {:?} because of {:?}",
                id, video_str, e
            );
        }

        Ok(())
    }

    pub fn get_video(&mut self, id: &str) -> Result<Video, StoreError> {
        if self.conn.is_none() {
            return Err(StoreError::client_error());
        }
        let conn = self.conn.as_mut().unwrap();

        match conn.get::<_, Option<String>>(&id) {
            Ok(Some(v)) => Ok(Video::from(v)),
            Ok(None) => Err(StoreError::NotFound),
            Err(e) => Err(StoreError::Internal(e)),
        }
    }

    pub fn del_video(&mut self, id: &str) -> Result<(), StoreError> {
        if self.conn.is_none() {
            return Err(StoreError::client_error());
        }
        let conn = self.conn.as_mut().unwrap();
        if let Err(e) = conn.del::<_, ()>(id) {
            warn!("Unable to delete {:?} because of {}", id, e);
        }

        Ok(())
    }
}
