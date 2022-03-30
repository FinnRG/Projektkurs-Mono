use actix_cors::Cors;
use actix_web::{
    error::ParseError,
    get,
    http::header::Header,
    post,
    web::{Either, Form, Json},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use auth_lib::{
    create_jwt,
    db::{check_password, create_user, CreateUserError},
};
use serde::Deserialize;

fn create_auth_response(id: String) -> HttpResponse {
    let jwt = create_jwt(id);
    let credentials = Bearer::new(jwt);
    HttpResponse::Ok()
        .insert_header(("Authorization", credentials.to_string()))
        .finish()
}

#[derive(Deserialize)]
struct Register {
    name: String,
    email: String,
    password: String,
}

#[post("/register")]
async fn register(form: Either<Json<Register>, Form<Register>>) -> impl Responder {
    let Register {
        name,
        email,
        password,
    } = form.into_inner();
    let db_res = create_user(&name, &email, &password);

    match db_res {
        Ok(id) => create_auth_response(id),
        Err(e) => match e {
            CreateUserError::UniqueViolation => HttpResponse::Conflict().finish(),
            _ => HttpResponse::ServiceUnavailable().finish(),
        },
    }
}

#[derive(Deserialize)]
struct Login {
    email: String,
    password: String,
}

#[post("/login")]
async fn login(form: Either<Json<Login>, Form<Login>>) -> impl Responder {
    let Login { email, password } = form.into_inner();

    match check_password(&email, &password) {
        Some(id) => create_auth_response(id),
        _ => HttpResponse::Unauthorized().finish(),
    }
}

#[get("/id")]
async fn get_id(req: HttpRequest) -> Result<String, ParseError> {
    let auth = Authorization::<Bearer>::parse(&req)?;
    Ok(auth.as_ref().token().to_string())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(register)
            .service(login)
            .service(get_id)
            .wrap(Cors::permissive())
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
