use crate::*;
use models::{Comment, NewComment};
use serde::Serialize;

pub fn create_comment<'a>(
    conn: &PgConnection,
    user_id: &'a str,
    video_id: &'a str,
    content: &'a str,
) -> Comment {
    use schema::comments;

    let new_comment = NewComment {
        user_id,
        video_id,
        content,
    };

    diesel::insert_into(comments::table)
        .values(&new_comment)
        .get_result(conn)
        .expect("Error saving new comment")
}

pub fn get_comment_by_user(conn: &PgConnection, user_id: &str) -> Vec<Comment> {
    use crate::schema::users::dsl::users;
    use models::User;

    let user = users
        .find(user_id)
        .first::<User>(conn)
        .expect("User ID is not correct");

    Comment::belonging_to(&user)
        .load::<Comment>(conn)
        .expect("Unable to fetch comments for user")
}

#[derive(Queryable, Serialize)]
pub struct UserComment {
    content: String,
    name: String,
}

pub fn get_comment_by_video(conn: &PgConnection, video_id: &str) -> Vec<UserComment> {
    // use crate::schema::videos::dsl::videos;
    // use models::Video;
    use schema::comments;
    use schema::users;

    /*let video = videos
    .find(video_id)
    .first::<Video>(conn)
    .expect("Video ID is not correct");*/

    comments::table
        .inner_join(users::table)
        .filter(comments::video_id.eq(video_id))
        .select((comments::content, users::name))
        .load::<UserComment>(conn)
        .expect("Unable to fetch comments for video")

    /*Comment::belonging_to(&video)
    .load::<Comment>(conn)
    .expect("Unable to fetch comment for video")*/
}
