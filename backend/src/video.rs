use crate::PostgresConn;
use postgres::models::Video;
use postgres::video::*;
use rocket::serde::json::Json;
use rocket::Route;

#[get("/list")]
async fn list(conn: PostgresConn) -> Json<Vec<Video>> {
    Json(conn.run(move |c| list_videos(c)).await)
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
    routes![list]
}
