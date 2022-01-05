use crate::*;
use diesel::pg::upsert::*;
use models::{NewTag, Tag};
use serde::Serialize;

pub fn create_tag<'a>(
    conn: &PgConnection,
    name: &'a str,
    description: Option<&'a str>,
    author: &'a str,
) {
    use schema::tags;

    let new_tag = NewTag {
        name,
        description: description.unwrap_or(""),
        author,
    };

    diesel::insert_into(tags::table)
        .values(&new_tag)
        .execute(conn)
        .expect("Error creating new tag");
}

pub fn add_tag_to_video(conn: &PgConnection, tag_id: i32, video_id: &str) {
    use schema::tag_to_video;

    diesel::insert_into(tag_to_video::table)
        .values((
            tag_to_video::tag_id.eq(tag_id),
            tag_to_video::video_id.eq(video_id),
        ))
        .on_conflict(on_constraint("tag_to_video_pkey"))
        .do_nothing()
        .execute(conn)
        .expect("Error saving new tag to video relationship");
}

pub fn remove_tag_from_video(conn: &PgConnection, tag_id: i32, video_id: &str) {
    use schema::tag_to_video;

    diesel::delete(tag_to_video::table.find((tag_id, video_id)))
        .execute(conn)
        .expect("Unable to remove relationship for tag and user");
}

pub fn soft_delete_tag(conn: &PgConnection, tag_id: i32) {
    use schema::tags::dsl::*;

    diesel::update(tags.find(tag_id))
        .set(deleted.eq(true))
        .execute(conn)
        .expect("Unable to set delete to TRUE for tag");
}

pub fn hard_delete_tag(conn: &PgConnection, tag_id: i32) {
    use schema::tag_to_video;
    use schema::tags;

    diesel::delete(tag_to_video::table.filter(tag_to_video::tag_id.eq(tag_id)))
        .execute(conn)
        .expect("Unable to delete relationships with tag from tag_to_video");

    diesel::delete(tags::table.find(tag_id))
        .execute(conn)
        .expect("Unable to delete tag with tag_id from tags");
}

pub fn restore_tag(conn: &PgConnection, tag_id: i32) {
    use schema::tags;

    diesel::update(tags::table.find(tag_id))
        .set(tags::deleted.eq(false))
        .execute(conn)
        .expect("Unable to restore tag");
}

#[derive(Queryable, Serialize)]
pub struct VideoTag {
    id: i32,
    name: String,
    description: Option<String>,
    deleted: bool,
}

pub fn get_tags_for_video(conn: &PgConnection, video_id: &str) -> Vec<VideoTag> {
    use schema::tag_to_video;
    use schema::tags;

    tag_to_video::table
        .filter(tag_to_video::video_id.eq(video_id))
        .inner_join(tags::table)
        .select((
            tag_to_video::tag_id,
            tags::name,
            tags::description,
            tags::deleted,
        ))
        .load::<VideoTag>(conn)
        .expect("Unable to fetch tags for video")
}
