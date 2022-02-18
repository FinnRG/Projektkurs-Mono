use crate::PostgresConn;
use postgres::user::*;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::Route;
use uuid::Uuid;

#[post("/register?<username>&<email>&<password>")]
async fn register(
    conn: PostgresConn,
    cookies: &CookieJar<'_>,
    username: String,
    email: String,
    password: String,
) -> Status {
    let id = Uuid::new_v4().to_string();
    let new_user = conn
        .run(move |c| create_user(c, &id, &username, &email, &password))
        .await;
    cookies.add_private(Cookie::new("user_id", new_user.id));

    Status::Ok
}

#[post("/login?<email>&<password>")]
async fn login(
    conn: PostgresConn,
    cookies: &CookieJar<'_>,
    email: String,
    password: String,
) -> Status {
    match conn
        .run(move |c| check_password(c, &email, &password))
        .await
    {
        Some(x) => {
            cookies.add_private(Cookie::new("user_id", x));
            Status::Ok
        }
        None => Status::Unauthorized,
    }
}

#[post("/logout")]
async fn logout(cookies: &CookieJar<'_>) {
    if let Some(crumb) = cookies.get_private("user_id") {
        cookies.remove_private(crumb)
    }
}

#[get("/id", format = "text/html")]
fn id(cookies: &CookieJar<'_>) -> Option<String> {
    cookies
        .get_private("user_id")
        .map(|crumb| format!("User Id: {}", crumb.value()))
}

pub fn routes() -> Vec<Route> {
    routes![register, login, logout, id]
}
