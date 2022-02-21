use crate::util::get_user_id;
use crate::PostgresConn;
use postgres::models::{Playlist, PlaylistEntry};
use postgres::playlist::*;
use rocket::http::{CookieJar, Status};
use rocket::serde::json::Json;
use rocket::Route;
use uuid::Uuid;

#[post("/create?<title>")]
async fn create(conn: PostgresConn, cookies: &CookieJar<'_>, title: String) -> Status {
    let user_id = get_user_id!(cookies);
    let id = Uuid::new_v4().to_string();

    conn.run(move |c| create_playlist(c, &id, &title, &user_id))
        .await;

    Status::Ok
}

#[post("/add?<playlist_id>&<video_id>")]
async fn add_video(
    conn: PostgresConn,
    cookies: &CookieJar<'_>,
    playlist_id: String,
    video_id: String,
) -> Status {
    let user_id = get_user_id!(cookies);

    conn.run(move |c| add_video_to_playlist(c, &playlist_id, &video_id, &user_id))
        .await;

    Status::Ok
}

#[post("/remove?<playlist_id>&<entry_id>")]
async fn remove_video(
    conn: PostgresConn,
    cookies: &CookieJar<'_>,
    playlist_id: String,
    entry_id: i32,
) -> Status {
    let user_id = get_user_id!(cookies);

    conn.run(move |c| remove_video_from_playlist(c, &playlist_id, entry_id, &user_id))
        .await;

    Status::Ok
}

#[post("/delete?<playlist_id>")]
async fn delete(conn: PostgresConn, cookies: &CookieJar<'_>, playlist_id: String) -> Status {
    let user_id = get_user_id!(cookies);

    conn.run(move |c| delete_playlist(c, &playlist_id, &user_id))
        .await;

    Status::Ok
}

#[get("/get")]
async fn get(conn: PostgresConn, cookies: &CookieJar<'_>) -> Option<Json<Vec<Playlist>>> {
    let user_id = get_user_id!(cookies, None);

    Some(Json(conn.run(move |c| get_playlists(c, &user_id)).await))
}

#[get("/get?<playlist_id>")]
async fn get_individual(
    conn: PostgresConn,
    cookies: &CookieJar<'_>,
    playlist_id: String,
) -> Option<Json<Vec<PlaylistEntry>>> {
    let user_id = get_user_id!(cookies, None);

    Some(Json(
        conn.run(move |c| get_videos_for_playlist(c, &playlist_id, &user_id))
            .await,
    ))
}

#[get("/info?<playlist_id>")]
async fn info(
    conn: PostgresConn,
    cookies: &CookieJar<'_>,
    playlist_id: String,
) -> Option<Json<Playlist>> {
    let user_id = get_user_id!(cookies, None);

    let playlist = conn
        .run(move |c| get_playlist_info(c, &playlist_id, &user_id))
        .await;

    if playlist.is_none() {
        return None;
    }

    Some(Json(playlist.unwrap()))
}

#[post("/update?<playlist_id>&<title>")]
async fn update(
    conn: PostgresConn,
    cookies: &CookieJar<'_>,
    playlist_id: String,
    title: Option<String>,
) -> Status {
    let user_id = get_user_id!(cookies);

    conn.run(move |c| update_playlist(c, &playlist_id, title.as_deref(), &user_id))
        .await;

    Status::Ok
}

pub fn routes() -> Vec<Route> {
    routes![
        create,
        add_video,
        remove_video,
        delete,
        get,
        get_individual,
        info,
        update
    ]
}
