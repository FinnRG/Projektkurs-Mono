use crate::util::get_user_id;
use crate::PostgresConn;
use postgres::like::*;
use rocket::{
    http::{CookieJar, Status},
    serde::json::Json,
    Route,
};

#[post("/add?<video_id>&<value>")]
async fn add(conn: PostgresConn, cookies: &CookieJar<'_>, video_id: String, value: bool) -> Status {
    let user_id = get_user_id!(cookies);

    conn.run(move |c| add_like(c, &user_id, &video_id, value))
        .await;

    Status::from_code(200).unwrap()
}

#[post("/remove?<video_id>")]
async fn remove(conn: PostgresConn, cookies: &CookieJar<'_>, video_id: String) -> Status {
    let user_id = get_user_id!(cookies);

    conn.run(move |c| delete_like(c, &user_id, &video_id)).await;

    Status::from_code(200).unwrap()
}

#[get("/info?<video_id>")]
async fn info(conn: PostgresConn, cookies: &CookieJar<'_>, video_id: String) -> Json<VideoRating> {
    let user_id = cookies
        .get_private("user_id")
        .map_or(None, |crumb| Some(crumb.value().to_owned()));
    Json(
        conn.run(move |c| info_like(c, user_id.as_deref(), &video_id))
            .await,
    )
}

pub fn routes() -> Vec<Route> {
    routes![add, info, remove]
}
