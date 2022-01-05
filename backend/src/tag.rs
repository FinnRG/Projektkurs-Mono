use crate::get_user_id;
use crate::PostgresConn;
use postgres::tag::*;
use rocket::http::{CookieJar, Status};
use rocket::serde::json::Json;
use rocket::Route;

#[post("/create?<name>&<description>")]
async fn create(
    conn: PostgresConn,
    cookies: &CookieJar<'_>,
    name: String,
    description: Option<String>,
) -> Status {
    let user_id = get_user_id!(cookies);

    conn.run(move |c| create_tag(c, &name, description.as_deref(), &user_id))
        .await;

    Status::from_code(200).unwrap()
}

#[post("/add?<tag_id>&<video_id>")]
async fn add_to_video(
    conn: PostgresConn,
    cookies: &CookieJar<'_>,
    tag_id: i32,
    video_id: String,
) -> Status {
    // Checks that the user is logged in, serves no real purpose
    let _user_id = get_user_id!(cookies);

    conn.run(move |c| add_tag_to_video(c, tag_id, &video_id))
        .await;

    Status::from_code(200).unwrap()
}

#[post("/remove?<tag_id>&<video_id>")]
async fn remove_from_video(conn: PostgresConn, cookies: &CookieJar<'_>, tag_id: i32, video_id: String) -> Status {
    // Checks that the user is logged in, serves no real purpose
    let _user_id = get_user_id!(cookies);

    conn.run(move |c| remove_tag_from_video(c, tag_id, &video_id))
        .await;

    Status::from_code(200).unwrap()
}

#[get("/get?<video_id>")]
async fn get_from_video(conn: PostgresConn, video_id: String) -> Json<Vec<VideoTag>> {
    Json(conn.run(move |c| get_tags_for_video(c, &video_id)).await)
}

#[post("/delete?<tag_id>")]
async fn soft_delete(conn: PostgresConn, cookies: &CookieJar<'_>, tag_id: i32) -> Status {
    // Checks that the user is logged in, serves no real purpose
    let _user_id = get_user_id!(cookies);

    conn.run(move |c| soft_delete_tag(c, tag_id)).await;

    Status::from_code(200).unwrap()
}

#[post("/delete_full?<tag_id>")]
async fn hard_delete(conn: PostgresConn, cookies: &CookieJar<'_>, tag_id: i32) -> Status {
    // Checks that the user is logged in, serves no real purpose
    let _user_id = get_user_id!(cookies);

    conn.run(move |c| hard_delete_tag(c, tag_id)).await;

    Status::from_code(200).unwrap()
}

#[post("/restore?<tag_id>")]
async fn restore(conn: PostgresConn, cookies: &CookieJar<'_>, tag_id: i32) -> Status {
    // Checks that the user is logged in, serves no real purpose
    let _user_id = get_user_id!(cookies);

    conn.run(move |c| restore_tag(c, tag_id)).await;

    Status::from_code(200).unwrap()
}

pub fn routes() -> Vec<Route> {
    routes![
        add_to_video,
        create,
        get_from_video,
        hard_delete,
        remove_from_video,
        restore,
        soft_delete
    ]
}
