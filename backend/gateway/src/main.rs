use actix_cors::Cors;
use actix_web::{dev::ServiceRequest, get, web, App, Error, HttpResponse, HttpServer, Responder};
use actix_web_httpauth::{extractors::bearer::BearerAuth, middleware::HttpAuthentication};
use rpc::auth_client::AuthClient;
use rpc::LoginRequest;

mod rpc {
    tonic::include_proto!("auth");
}

#[get("/login")]
async fn login() -> Result<impl Responder, > {
    let mut client = AuthClient::connect("http://auth:50051").await?;

    let request = tonic::Request::new(LoginRequest {
        email: "".into(),
        password: "".into(),
    });

    let response = client.login(request).await?;

    Ok(HttpResponse::Ok().body("test"))

    // response.into_inner().jwt
}

async fn ok_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    eprintln!("{:?}", credentials);
    Ok(req)
}

// TODO: Add logging
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    HttpServer::new(|| {
        let cors_middleware = Cors::permissive();

        let authentication_middleware = HttpAuthentication::bearer(ok_validator);

        App::new()
            .wrap(cors_middleware)
            .wrap(authentication_middleware)
            .service(login)
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
