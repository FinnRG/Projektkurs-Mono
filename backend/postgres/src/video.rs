use crate::*;
use models::{NewVideo, Video, VideoUpdate};

pub fn create_video<'a>(
    conn: &PgConnection,
    id: &'a str,
    user_id: &'a str,
    title: &'a str,
    description: &'a str,
) {
    use schema::videos;

    let new_video = NewVideo {
        id,
        user_id,
        title,
        description,
    };

    diesel::insert_into(videos::table)
        .values(&new_video)
        .execute(conn)
        .expect("Error saving new video");
}

pub fn get_video(conn: &PgConnection, video_id: &str) -> Video {
    use schema::videos;

    videos::table.find(video_id)
        .first::<Video>(conn)
        .expect("Unable to get video info")
}

pub fn update_video<'a>(conn: &PgConnection, id: &'a str, title: Option<&'a str>, description: Option<&'a str>) {
    use schema::videos;

    diesel::update(videos::table.find(id))
        .set(VideoUpdate { title, description })
        .execute(conn)
        .expect("Unable to update video");
}

pub fn list_videos(conn: &PgConnection) -> Vec<Video> {
    schema::videos::table
        .load::<Video>(conn)
        .expect("Could not load videos")
}
