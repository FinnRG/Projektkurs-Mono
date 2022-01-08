use crate::schema::{comments, likes, tags, users, videos};
use serde::Serialize;

#[derive(Queryable, Identifiable, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Queryable, Identifiable, Associations, Debug, Serialize)]
#[belongs_to(User)]
pub struct Video {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub description: String,
}

#[derive(Insertable)]
#[table_name = "videos"]
pub struct NewVideo<'a> {
    pub id: &'a str,
    pub user_id: &'a str,
    pub title: &'a str,
    pub description: &'a str,
}

#[derive(Queryable, Identifiable, Associations, Debug, Serialize)]
#[belongs_to(User)]
#[belongs_to(Video)]
pub struct Comment {
    pub id: i32,
    pub user_id: String,
    pub video_id: String,
    pub content: String,
}

#[derive(Insertable)]
#[table_name = "comments"]
pub struct NewComment<'a> {
    pub user_id: &'a str,
    pub video_id: &'a str,
    pub content: &'a str,
}

#[derive(Queryable, Identifiable, Associations, Debug, Serialize)]
#[belongs_to(User)]
#[belongs_to(Video)]
#[primary_key(user_id, video_id)]
pub struct Like {
    pub value: bool,
    pub user_id: String,
    pub video_id: String,
}

#[derive(Insertable)]
#[table_name = "likes"]
pub struct NewLike<'a> {
    pub value: bool,
    pub user_id: &'a str,
    pub video_id: &'a str,
}

#[derive(Queryable, Identifiable, Associations, Debug, Serialize)]
#[belongs_to(User, foreign_key = "author")]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub author: String,
    pub deleted: bool,
}

#[derive(Insertable)]
#[table_name = "tags"]
pub struct NewTag<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub author: &'a str,
}

#[derive(AsChangeset)]
#[table_name = "tags"]
pub struct TagUpdate<'a> {
    pub name: Option<&'a str>,
    pub description: Option<&'a str>,
}
