extern crate bcrypt;

use postgres::user::*;
use rocket::Route;
use rocket::form::{Form, Strict};
use postgres::{establish_connection};
use rocket::serde::{Serialize, json::Json};
use rocket::http::{Cookie, CookieJar, Status};
use rocket::response::{Flash, Redirect};
use uuid::Uuid;

#[derive(FromForm)]
struct CreateUser<'r> {
    name: &'r str,
    email: &'r str,
    password: &'r str
}


#[derive(FromForm)]
struct LoginUser<'r> {
    email: &'r str,
    password: &'r str
}

#[post("/register", data = "<user>")]
fn register(cookies: &CookieJar<'_>, user: Form<Strict<CreateUser<'_>>>) -> Flash<Redirect> {
    let conn = establish_connection();

    let id = Uuid::new_v4().to_string();

    let new_user = create_user(&conn, &id, user.name, user.email, user.password);
    cookies.add_private(Cookie::new("user_id", id));

    Flash::success(Redirect::to("/player"), "Successfully created a new user.")
}

#[post("/login", data = "<user>")]
fn login(cookies: &CookieJar<'_>, user: Form<Strict<LoginUser<'_>>>) -> Status {
    let conn = establish_connection();
    println!("Email: {}, Password: {}", user.email, user.password);

    match check_password(&conn, user.email, user.password) {
        Some(x) => {
            cookies.add_private(Cookie::new("user_id", x));
            println!("Cookie added!");
            Status::from_code(200).unwrap()
        },
        None => Status::from_code(500).unwrap()
    }
}

#[get("/id", format="text/html")]
fn id(cookies: &CookieJar<'_>) -> Option<String> {
    cookies.get_private("user_id")
        .map(|crumb| format!("User Id: {}", crumb.value()))
}

pub fn routes() -> Vec<Route> {
    routes![register, login, id]
}