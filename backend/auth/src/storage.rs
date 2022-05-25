use crate::{user::User, POOL};
use log::warn;
use r2d2::PooledConnection;
use redis::Commands;
use redis::{ErrorKind, RedisError};
use tonic::Status;

pub struct Store {
    conn: Option<PooledConnection<redis::Client>>,
}

pub enum StoreError {
    NotFound,
    Internal(RedisError),
}

impl StoreError {
    fn client_error() -> StoreError {
        StoreError::Internal(RedisError::from((ErrorKind::ClientError, "Clienterror")))
    }
}

impl Store {
    pub fn new() -> Self {
        Store {
            conn: POOL.get().ok(),
        }
    }

    pub fn get_user(&mut self, email: &str) -> Result<User, StoreError> {
        if self.conn.is_none() {
            return Err(StoreError::client_error());
        }
        let conn = self.conn.as_mut().unwrap();
        match conn.get::<_, Option<String>>(email) {
            Ok(Some(user)) => Ok(User::from_json(&user)),
            Ok(None) => Err(StoreError::NotFound),
            Err(e) => Err(StoreError::Internal(e)),
        }
    }

    // TODO: Get user email with id and THEN delete password with associated email
    pub fn del_user(&mut self, email: &str) -> Result<(), StoreError> {
        if self.conn.is_none() {
            return Err(StoreError::client_error());
        }
        let conn = self.conn.as_mut().unwrap();
        if let Err(e) = conn.del::<_, ()>(email) {
            return Err(StoreError::Internal(e));
        }

        Ok(())
    }

    pub fn set_user(&mut self, user: &User) -> Result<(), StoreError> {
        if self.conn.is_none() {
            return Err(StoreError::client_error());
        }
        let conn = self.conn.as_mut().unwrap();
        let user_str = user.to_json();
        if let Err(e) = conn.set::<_, _, ()>(&user.email, &user_str) {
            warn!(
                "Unable to update {} with {:?} because of {:?}",
                &user.email, &user_str, e
            );
        }

        Ok(())
    }
}
