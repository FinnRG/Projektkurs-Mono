use crate::schema::{users, videos};
use serde::Serialize;

#[derive(Queryable, Identifiable, Debug, PartialEq)]
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

#[derive(Queryable, Identifiable, Associations, Debug, PartialEq, Serialize)]
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
