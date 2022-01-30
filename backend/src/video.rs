use crate::redis::CacheHelper;
use crate::util::{cache_json, get_user_id};
use crate::PostgresConn;
use postgres::models::Video;
use postgres::tag::{get_tags_for_video, TagForVideo};
use postgres::video::*;
use rocket::http::{CookieJar, Status};
use rocket::serde::json::Json;
use rocket::Route;
use serde_json;

#[get("/get?<video_id>")]
async fn get(conn: PostgresConn, video_id: String) -> Json<Video> {
    Json(conn.run(move |c| get_video(c, &video_id)).await)
}

#[get("/list")]
async fn list(cache_helper: CacheHelper, conn: PostgresConn) -> String {
    cache_json!(cache_helper, &conn.run(move |c| list_videos(c)).await)
}

// Json<Vec<TagForVideo>>
#[get("/tags?<video_id>")]
async fn get_tags(cache_helper: CacheHelper, conn: PostgresConn, video_id: String) -> String {
    cache_json!(
        cache_helper,
        &conn.run(move |c| get_tags_for_video(c, &video_id)).await
    )
}

#[post("/update?<video_id>&<title>&<description>")]
async fn update<'a>(
    conn: PostgresConn,
    cookies: &CookieJar<'_>,
    video_id: String,
    title: Option<String>,
    description: Option<String>,
) -> Status {
    let _user_id = get_user_id!(cookies);

    conn.run(move |c| update_video(c, &video_id, title.as_deref(), description.as_deref()))
        .await;

    Status::from_code(200).unwrap()
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
    routes![get, get_tags, list, update]
}
