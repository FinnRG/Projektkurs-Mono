use actix_cors::Cors;
use actix_web::{get, App, HttpServer, Responder, HttpRequest};
use actix_web_httpauth::middleware::HttpAuthentication;
use auth_lib::parse_request;

#[actix::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().wrap(Cors::permissive()))
        .bind(("0.0.0.0", 8000))?
        .run()
        .await
}
