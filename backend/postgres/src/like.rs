use crate::*;
use diesel::pg::upsert::*;
use models::{Like, NewLike};
use serde::Serialize;

pub fn add_like<'a>(conn: &PgConnection, user_id: &'a str, video_id: &'a str, value: bool) -> Like {
    use schema::likes;

    let new_like = NewLike {
        value,
        user_id,
        video_id,
    };

    // Update or insert
    diesel::insert_into(likes::table)
        .values(&new_like)
        .on_conflict(on_constraint("likes_pkey"))
        .do_update()
        .set(likes::value.eq(value))
        .get_result(conn)
        .expect("Error saving new like")
}

pub fn delete_like(conn: &PgConnection, user_id: &str, video_id: &str) {
    use schema::likes;

    diesel::delete(likes::table.find((user_id, video_id)))
        .execute(conn)
        .expect("Unable to delete comment for user and video");
}

#[allow(dead_code)]
#[derive(Serialize)]
pub struct VideoRating {
    user_like: Option<bool>,
    likes: usize,
    dislikes: usize,
}

pub fn info_like(conn: &PgConnection, user_id: Option<&str>, video_id: &str) -> VideoRating {
    match user_id {
        Some(id) => get_video_ratings_for_user(conn, id, video_id),
        None => get_video_ratings_for_guest(conn, video_id),
    }
}

fn get_video_ratings_for_guest(conn: &PgConnection, video_id: &str) -> VideoRating {
    use schema::likes;

    let mut likes = 0;
    let mut dislikes = 0;

    let like_data = likes::table
        .filter(likes::video_id.eq(video_id))
        .select(likes::value)
        .load::<bool>(conn)
        .expect("Unable to fetch likes for video");

    like_data.iter().for_each(|like| {
        match like {
            true => likes += 1,
            false => dislikes += 1,
        };
    });

    VideoRating {
        user_like: None,
        likes,
        dislikes,
    }
}

fn get_video_ratings_for_user(conn: &PgConnection, user_id: &str, video_id: &str) -> VideoRating {
    use schema::likes;

    let mut user_like = None;
    let mut likes = 0;
    let mut dislikes = 0;

    let like_data = likes::table
        .filter(likes::video_id.eq(video_id))
        .select((likes::value, likes::user_id))
        .load::<(bool, String)>(conn)
        .expect("Unable to fetch likes for video");

    like_data.iter().for_each(|like| {
        match like.0 {
            true => likes += 1,
            false => dislikes += 1,
        };
        if like.1 == user_id {
            user_like = Some(like.0);
        }
    });

    VideoRating {
        user_like,
        likes,
        dislikes,
    }
}
