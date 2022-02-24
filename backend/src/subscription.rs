use postgres::subscription::*;
use rocket::{
    http::{CookieJar, Status},
    Route,
};

use crate::{get_user_id, PostgresConn};

#[post("/new?<tag_id>")]
async fn new(conn: PostgresConn, cookies: &CookieJar<'_>, tag_id: i32) -> Status {
    let user_id = get_user_id!(cookies);

    conn.run(move |c| new_subscription(c, &user_id, tag_id))
        .await;

    Status::Ok
}

#[post("/delete?<tag_id>")]
async fn delete(conn: PostgresConn, cookies: &CookieJar<'_>, tag_id: i32) -> Status {
    let user_id = get_user_id!(cookies);

    conn.run(move |c| delete_subscription(c, &user_id, tag_id))
        .await;

    Status::Ok
}

pub fn routes() -> Vec<Route> {
    routes![new, delete]
}
