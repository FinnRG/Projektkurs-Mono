use crate::schema::{comments, users, videos};
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
