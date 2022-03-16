use crate::util::get_user_id;
use crate::PostgresConn;
use msostream::comment::*;
use rocket::http::{CookieJar, Status};
use rocket::serde::json::Json;
use rocket::Route;

#[post("/create?<video_id>&<content>")]
async fn create(
    conn: PostgresConn,
    cookies: &CookieJar<'_>,
    video_id: String,
    content: String,
) -> Status {
    let user_id = get_user_id!(cookies);

    conn.run(move |c| create_comment(c, &user_id, &video_id, &content))
        .await;

    Status::Ok
}

#[get("/get?<video_id>")]
async fn get(conn: PostgresConn, video_id: String) -> Json<Vec<UserComment>> {
    Json(conn.run(move |c| get_comment_by_video(c, &video_id)).await)
}

#[post("/delete?<comment_id>")]
async fn delete(conn: PostgresConn, cookies: &CookieJar<'_>, comment_id: i32) -> Status {
    let user_id = get_user_id!(cookies);

    conn.run(move |c| delete_comment(c, &user_id, comment_id))
        .await;

    Status::Ok
}

pub fn routes() -> Vec<Route> {
    routes![create, delete, get]
}
