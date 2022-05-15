use crate::{
    construct_response,
    kafka::{self, emit_user},
    users::v1::{
        register_response::Result as RegisterResult, RegisterRequest, RegisterResponse,
        UserRegisteredEvent,
    },
};
use argon2::Config;
use rdkafka::{message::ToBytes, producer::future_producer::OwnedDeliveryResult};
use tonic::{Response, Status};

pub async fn handle_register_request(
    req: RegisterRequest,
) -> Result<Response<RegisterResponse>, Status> {
    if validate_request(&req) {
        return Err(Status::invalid_argument(
            "name, email and password must be specified",
        ));
    }
    let id = uuid::Uuid::new_v4();

    let hash = generate_hash(&req.password);

    if emit_registered_event(req, &id.to_string(), hash)
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

fn validate_request(req: &RegisterRequest) -> bool {
    req.name.is_empty() || req.email.is_empty() || req.password.is_empty()
}

// Generates the hash using Argon2 and a random salt using openssl
fn generate_hash(password: &str) -> String {
    let mut buf = [0u8; 128];
    openssl::rand::rand_bytes(&mut buf).expect("Unable to generate random salt");
    let config = Config::default();
    argon2::hash_encoded(password.to_bytes(), &buf, &config).expect("Unable to hash password")
}

async fn emit_registered_event(
    user: RegisterRequest,
    id: &str,
    hash: String,
) -> OwnedDeliveryResult {
    let event = UserRegisteredEvent {
        id: id.to_string(),
        email: user.email,
        password: hash,
        name: user.name,
    };
    emit_user(id, &kafka::UserEvent::Registered(event)).await
}
