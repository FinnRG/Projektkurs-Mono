use crate::*;
use models::{Video, NewVideo};
use diesel::prelude;

pub fn create_video<'a>(conn: &PgConnection, id: &'a str, user_id: i32, title: &'a str, description: &'a str) {
    use schema::videos;

    let new_video = NewVideo {
        id,
        user_id,
        title,
        description,
    };

    diesel::insert_into(videos::table)
        .values(&new_video)
        .get_result(conn)
        .expect("Error saving new video")
}