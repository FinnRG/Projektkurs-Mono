use crate::{
    construct_response,
    kafka::{self, emit_user},
    storage::{
        models::NewDBUser,
        Store,
    },
    users::v1::{
        register_response::Result as RegisterResult, RegisterRequest, RegisterResponse,
        UserRegisteredEvent,
    },
};
use argon2::Config;
use opentelemetry::trace::TraceContextExt;
use rdkafka::{message::ToBytes, producer::future_producer::OwnedDeliveryResult};
use tonic::{Response, Status};
use tracing_opentelemetry::OpenTelemetrySpanExt;
use uuid::Uuid;

#[tracing::instrument]
pub async fn handle_register_request(
    req: RegisterRequest,
) -> Result<Response<RegisterResponse>, Status> {
    let id = tracing::Span::current()
        .context()
        .span()
        .span_context()
        .trace_id();
    tracing::info!("handling login request, TraceId: {}", id);

    if is_invalid_request(&req) {
        return Err(Status::invalid_argument(
            "name, email and password must be specified",
        ));
    }

    let id = uuid::Uuid::new_v4();
    tracing::debug!("Generated user id: {}", id.to_string());

    let hash = generate_hash(&req.password);

    let mut store = Store::new();
    let id = store
        .create_user(&NewDBUser::from(&req))
        .expect("Unable to save user");

    if emit_registered_event(req, &id.to_string(), hash)
        .await
        .is_err()
    {
        tracing::error!("Failed to connet to kafka");
        return Err(Status::internal("Internal kafka error"));
    }

    Ok(construct_register_response(id))
}

fn construct_register_response(id: Uuid) -> Response<RegisterResponse> {
    let mut resp = Response::new(RegisterResponse {
        res: RegisterResult::Accepted as i32,
    });

    construct_response(&mut resp, &id.to_string());

    resp
}

fn is_invalid_request(req: &RegisterRequest) -> bool {
    req.name.is_empty() || req.email.is_empty() || req.password.is_empty()
}

// Generates the hash using Argon2 and a random salt using openssl
#[tracing::instrument]
fn generate_hash(password: &str) -> String {
    let mut buf = [0u8; 128];
    openssl::rand::rand_bytes(&mut buf).expect("Unable to generate random salt");
    let config = Config::default();
    argon2::hash_encoded(password.to_bytes(), &buf, &config).expect("Unable to hash password")
}

impl<'a> From<&'a RegisterRequest> for NewDBUser<'a> {
    fn from(item: &'a RegisterRequest) -> Self {
        NewDBUser {
            name: &item.name,
            email: &item.email,
            password: &item.password,
        }
    }
}

#[tracing::instrument]
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
