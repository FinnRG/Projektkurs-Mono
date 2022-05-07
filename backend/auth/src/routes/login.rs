use crate::construct_response;
use crate::storage::{Store, StoreError};
use crate::user::User;
use crate::users::v1::{LoginRequest, LoginResponse};
use tonic::{Response, Status};

pub async fn handle_login_request(req: LoginRequest) -> Result<Response<LoginResponse>, Status> {
    let mut store = Store::new();

    match store.get_user(&req.email) {
        Ok(user) => verify_password(&user, req.password.as_bytes()),
        Err(StoreError::NotFound) => Err(Status::invalid_argument("Couldn't find email address")),
        Err(StoreError::Internal(_)) => Err(Status::internal("Internal Redis error")),
    }
}

fn verify_password(user: &User, pwd: &[u8]) -> Result<Response<LoginResponse>, Status> {
    match argon2::verify_encoded(&user.password, pwd) {
        Ok(true) => {
            let mut resp = Response::new(LoginResponse {});
            construct_response(&mut resp, &user.id);
            Ok(resp)
        }
        Ok(false) => Err(Status::invalid_argument("Wrong password")),
        Err(_) => Err(Status::internal("Internal hashing error")),
    }
}
