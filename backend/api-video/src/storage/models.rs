use super::schema::videos;
use crate::Video;
use diesel::Queryable;

#[derive(Queryable, Insertable)]
#[table_name = "videos"]
pub struct DBVideo {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub author: String,
    pub date: String,
    pub visibility: i32,
    pub status: i32,
}

impl From<Video> for DBVideo {
    fn from(item: Video) -> Self {
        DBVideo {
            id: item.id,
            title: item.title,
            description: Some(item.description),
            author: item.author,
            date: item.date,
            visibility: item.visibility,
            status: item.status,
        }
    }
}

impl From<DBVideo> for Video {
    fn from(item: DBVideo) -> Self {
        Video {
            id: item.id,
            title: item.title,
            description: item.description.unwrap_or_default(),
            author: item.author,
            date: item.date,
            visibility: item.visibility,
            status: item.status,
        }
    }
}
