use crate::{
    construct_response,
    kafka::{self, emit_user},
    user::User,
    users::v1::{register_response::Result as RegisterResult, RegisterRequest, RegisterResponse},
};
use argon2::Config;
use rdkafka::message::ToBytes;
use tonic::{Response, Status};

pub async fn handle_register_request(
    req: RegisterRequest,
) -> Result<Response<RegisterResponse>, Status> {
    if req.name.is_empty() || req.email.is_empty() || req.password.is_empty() {
        return Err(Status::invalid_argument(
            "name, email and password must be specified",
        ));
    }
    let id = uuid::Uuid::new_v4();

    let hash = generate_hash(req.password);

    // Stringify the user struct
    let user = User {
        id: id.to_string(),
        name: req.name,
        email: req.email,
        password: hash,
    }
    .to_json();

    // Emit UserRegistered event
    if emit_user(&id.to_string(), &user, kafka::UserEvents::Registered)
        .await
        .is_err()
    {
        return Err(Status::internal("Internal kafka error"));
    }
    let mut resp = Response::new(RegisterResponse {
        res: RegisterResult::Accepted as i32,
    });

    construct_response(&mut resp, &id.to_string());

    Ok(resp)
}

// Generates the hash using Argon2 and a random salt using openssl
fn generate_hash(password: String) -> String {
    let mut buf = [0u8; 128];
    openssl::rand::rand_bytes(&mut buf).expect("Unable to generate random salt");
    let config = Config::default();
    argon2::hash_encoded(password.to_bytes(), &buf, &config).expect("Unable to hash password")
}
