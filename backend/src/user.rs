use postgres::user::*;
use crate::PostgresConn;
use rocket::Route;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::response::{Flash, Redirect};
use uuid::Uuid;

#[post("/register?<username>&<email>&<password>")]
async fn register(conn: PostgresConn, cookies: &CookieJar<'_>, username: String, email: String, password: String) -> Flash<Redirect> {
    let id = Uuid::new_v4().to_string();
    let new_user = conn.run(move |c| create_user(c, &id, &username, &email, &password)).await;
    cookies.add_private(Cookie::new("user_id", new_user.id));

    Flash::success(Redirect::to("/player"), "Successfully created a new user.")
}

#[post("/login?<email>&<password>")]
async fn login(conn: PostgresConn, cookies: &CookieJar<'_>, email: String, password: String) -> Status {

    match conn.run(move |c| check_password(c, &email, &password)).await {
        Some(x) => {
            cookies.add_private(Cookie::new("user_id", x));
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