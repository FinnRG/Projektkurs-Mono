extern crate bcrypt;

use rocket::Route;
use rocket::form::{Form, Strict};
use bcrypt::{DEFAULT_COST, hash};
use postgres::{establish_connection, create_user};

#[derive(FromForm)]
struct User<'r> {
    name: &'r str,
    email: &'r str,
    password: &'r str
}

#[post("/create", data = "<user>")]
fn create(user: Form<Strict<User<'_>>>) {
    let conn = establish_connection();
    let hash = hash(&user.password, DEFAULT_COST).expect("Unable to hash");
    create_user(&conn, user.name, user.email, &hash);
}

pub fn routes() -> Vec<Route> {
    routes![create]
}