use crate::PostgresConn;
use postgres::models::Video;
use postgres::video::*;
use postgres::tag::{get_tags_for_video, TagForVideo};
use rocket::serde::json::Json;
use rocket::Route;

#[get("/list")]
async fn list(conn: PostgresConn) -> Json<Vec<Video>> {
    Json(conn.run(move |c| list_videos(c)).await)
}

#[get("/tags?<video_id>")]
async fn get_tags(conn: PostgresConn, video_id: String) -> Json<Vec<TagForVideo>> {
    Json(conn.run(move |c| get_tags_for_video(c, &video_id)).await)
}

// Currently used in src/upload.rs
pub async fn create(
    conn: PostgresConn,
    id: String,
    user_id: String,
    title: String,
    description: String,
) {
    conn.run(move |c| create_video(c, &id, &user_id, &title, &description))
        .await;
}

pub fn routes() -> Vec<Route> {
    routes![list, get_tags]
}
