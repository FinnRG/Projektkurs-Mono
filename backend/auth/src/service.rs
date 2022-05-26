use crate::{
    user::User,
    users::v1::{RegisterRequest, RegisterResponse},
};
use auth::create_jwt;
use opentelemetry::{
    global::{self},
    propagation::Extractor,
    sdk::{propagation::TraceContextPropagator, trace::Config, Resource},
    trace::TraceError,
    KeyValue,
};
use std::env;
use tonic::transport::Server;
use tonic::{Request, Response, Status};
use tracing::Instrument;
use tracing_subscriber::{prelude::*, util::SubscriberInitExt};
use users::v1::user_service_server::UserService;
use users::v1::user_service_server::UserServiceServer;
use users::v1::{LoginRequest, LoginResponse};

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

include!("../gen/mod.rs");

mod kafka;
mod routes;
mod storage;
mod user;

lazy_static! {
    static ref JWTSECRET: String = env::var("JWTSECRET").unwrap();
    static ref DATABASE_URL: String = env::var("DATABASE_URL").unwrap();
}

fn construct_response<'a, T>(resp: &'a mut Response<T>, id: &str) -> &'a Response<T> {
    resp.metadata_mut().insert(
        "authorization",
        create_jwt(id.to_string())
            .parse()
            .expect("Unable to convert id to Metadata value"),
    );
    resp
}

struct MetadataMap<'a>(&'a tonic::metadata::MetadataMap);

impl<'a> Extractor for MetadataMap<'a> {
    /// Get a value for a key from the MetadataMap.  If the value can't be converted to &str, returns None
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|metadata| metadata.to_str().ok())
    }

    /// Collect all the keys from the MetadataMap.
    fn keys(&self) -> Vec<&str> {
        self.0
            .keys()
            .map(|key| match key {
                tonic::metadata::KeyRef::Ascii(v) => v.as_str(),
                tonic::metadata::KeyRef::Binary(v) => v.as_str(),
            })
            .collect::<Vec<_>>()
    }
}

#[derive(Debug, Default)]
struct Users {}

#[tonic::async_trait]
impl UserService for Users {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        // Validate
        let request = request.into_inner();
        if request.name.is_empty() || request.email.is_empty() || request.password.is_empty() {
            return Err(Status::invalid_argument(
                "name, email and password must be specified",
            ));
        }

        let handle_result = routes::register::handle_register_request(request);

        handle_result
            .instrument(tracing::info_span!("register request"))
            .await
    }
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let request = request.into_inner();

        let handle_result = routes::login::handle_login_request(request);

        handle_result
            .instrument(tracing::info_span!("login request"))
            .await
    }
}

fn tracing_init() -> Result<(), TraceError> {
    global::set_text_map_propagator(TraceContextPropagator::new());

    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_collector_endpoint(
            std::env::var("JAEGER_ENDPOINT").expect("JEAGER_ENDPOINT should be set"),
        )
        .with_service_name("auth")
        .with_trace_config(
            Config::default().with_resource(Resource::new(vec![KeyValue::new(
                "exporter",
                "otlp-jaeger",
            )])),
        )
        .install_batch(opentelemetry::runtime::Tokio)?;
    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let stdout = tracing_subscriber::fmt::layer().json();
    tracing_subscriber::registry()
        .with(opentelemetry)
        .with(stdout)
        .try_init()
        .map_err(|e| TraceError::Other(Box::new(e)))?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init();

    let addr = "0.0.0.0:8080".parse()?;
    let users = Users::default();

    let _tracer = tracing_init()?;

    Server::builder()
        .add_service(UserServiceServer::new(users))
        .serve(addr)
        .await?;

    opentelemetry::global::shutdown_tracer_provider();
    Ok(())
}

embed_migrations!();

fn init() {
    use diesel::prelude::*;
    embedded_migrations::run(
        &PgConnection::establish(&DATABASE_URL).expect("Unable to establish DB connection"),
    )
    .expect("Unable to run DB migrations");
}
