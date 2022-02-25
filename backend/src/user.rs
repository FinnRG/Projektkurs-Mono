use crate::{get_user_id, PostgresConn};
use postgres::models::{Notification, Subscription};
use postgres::user::*;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::serde::json::Json;
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

#[get("/subscriptions")]
async fn subscriptions(
    conn: PostgresConn,
    cookies: &CookieJar<'_>,
) -> Option<Json<Vec<Subscription>>> {
    let user_id = get_user_id!(cookies, None);

    Some(Json(
        conn.run(move |c| get_subscriptions(c, &user_id)).await,
    ))
}

#[get("/notifications/list")]
async fn notifications(
    conn: PostgresConn,
    cookies: &CookieJar<'_>,
) -> Option<Json<Vec<Notification>>> {
    let user_id = get_user_id!(cookies, None);

    Some(Json(
        conn.run(move |c| get_notifications(c, &user_id)).await,
    ))
}

#[post("/notifications/remove?<tag_id>&<video_id>")]
async fn remove(
    conn: PostgresConn,
    cookies: &CookieJar<'_>,
    tag_id: i32,
    video_id: String,
) -> Status {
    let user_id = get_user_id!(cookies);

    conn.run(move |c| remove_notification(c, &user_id, tag_id, &video_id))
        .await;

    Status::Ok
}

#[get("/id", format = "text/html")]
fn id(cookies: &CookieJar<'_>) -> Option<String> {
    cookies
        .get_private("user_id")
        .map(|crumb| format!("User Id: {}", crumb.value()))
}

pub fn routes() -> Vec<Route> {
    routes![
        register,
        login,
        logout,
        id,
        subscriptions,
        remove,
        notifications
    ]
}
