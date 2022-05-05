use crate::videos::v1::Video;
use crate::POOL;
use log::{error, warn};
use r2d2::PooledConnection;
use redis::{Client, Commands, RedisError, ErrorKind};

pub struct Store {
    conn: Option<PooledConnection<Client>>,
}

pub enum GetVideoError {
    NotFound,
    Internal(RedisError),
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

    pub fn set_video(&mut self, video: &Video) {
        if self.conn.is_none() {
            return;
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
    }

    pub fn get_video(&mut self, id: &str) -> Result<Video, GetVideoError> {
        if self.conn.is_none() {
            return Err(GetVideoError::Internal(RedisError::from((ErrorKind::ClientError, "Clienterror"))));
        }
        let conn = self.conn.as_mut().unwrap();

        match conn.get::<_, Option<String>>(&id) {
            Ok(Some(v)) => Ok(Video::from(v)),
            Ok(None) => Err(GetVideoError::NotFound),
            Err(e) => Err(GetVideoError::Internal(e)),
        }
    }
}
