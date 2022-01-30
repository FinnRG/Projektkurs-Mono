use crate::util::{get_user_id, invalidate};
use crate::PostgresConn;
use postgres::models::Tag;
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
) -> Option<Json<Tag>> {
    let user_id = cookies.get_private("user_id");
    if user_id.is_some() {
        let id = user_id.unwrap().value().to_owned();
        return Some(Json(
            conn.run(move |c| create_tag(c, &name, description.as_deref(), &id))
                .await,
        ));
    }
    None
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

    invalidate!("/video/tags", "video_id", &video_id);
    conn.run(move |c| add_tag_to_video(c, tag_id, &video_id))
        .await;


    Status::from_code(200).unwrap()
}

#[post("/remove?<tag_id>&<video_id>")]
async fn remove_from_video(
    conn: PostgresConn,
    cookies: &CookieJar<'_>,
    tag_id: i32,
    video_id: String,
) -> Status {
    // Checks that the user is logged in, serves no real purpose
    let _user_id = get_user_id!(cookies);

    invalidate!("/video/tags", "video_id", &video_id);
    conn.run(move |c| remove_tag_from_video(c, tag_id, &video_id))
        .await;

    Status::from_code(200).unwrap()
}

#[get("/get")]
async fn get(conn: PostgresConn) -> Json<Vec<Tag>> {
    Json(conn.run(move |c| get_tags(c)).await)
}

#[get("/get?<tag_id>")]
async fn get_tag(conn: PostgresConn, tag_id: i32) -> Json<Tag> {
    Json(conn.run(move |c| get_tag_info(c, tag_id)).await)
}

#[get("/videos?<tag_id>")]
async fn get_videos(conn: PostgresConn, tag_id: i32) -> Json<Vec<VideoForTag>> {
    Json(conn.run(move |c| get_videos_for_tag(c, tag_id)).await)
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

#[post("/update?<tag_id>&<name>&<description>")]
async fn update(
    conn: PostgresConn,
    cookies: &CookieJar<'_>,
    tag_id: i32,
    name: Option<String>,
    description: Option<String>,
) -> Status {
    // Checks that the user is logged in, serves no real purpose
    let _user_id = get_user_id!(cookies);

    conn.run(move |c| update_tag(c, tag_id, name.as_deref(), description.as_deref()))
        .await;

    Status::from_code(200).unwrap()
}

pub fn routes() -> Vec<Route> {
    routes![
        add_to_video,
        create,
        get,
        get_tag,
        get_videos,
        hard_delete,
        remove_from_video,
        restore,
        soft_delete,
        update
    ]
}
