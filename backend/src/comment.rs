use crate::PostgresConn;
use postgres::comment::UserComment;
use postgres::comment::{create_comment, get_comment_by_video};
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
    let user_id = match cookies.get_private("user_id") {
        Some(id) => id.value().to_owned(),
        None => return Status::from_code(401).unwrap(),
    };

    conn.run(move |c| create_comment(c, &user_id, &video_id, &content))
        .await;

    Status::from_code(200).unwrap()
}

#[get("/get?<video_id>")]
async fn get(conn: PostgresConn, video_id: String) -> Json<Vec<UserComment>> {
    Json(conn.run(move |c| get_comment_by_video(c, &video_id)).await)
}

pub fn routes() -> Vec<Route> {
    routes![create, get]
}
