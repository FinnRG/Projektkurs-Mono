use crate::schema::{users, videos};

#[derive(Queryable, Identifiable, Debug, PartialEq)]
pub struct User {
    pub name: String,
    pub email: String,
    pub password: String,
    pub id: String,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub id: &'a str,
}

#[derive(Queryable, Identifiable, Associations, Debug, PartialEq)]
#[belongs_to(User)]
pub struct Video {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub description: String,
}

#[derive(Insertable)]
#[table_name="videos"]
pub struct NewVideo<'a> {
    pub id: &'a str,
    pub user_id:  &'a str,
    pub title: &'a str,
    pub description: &'a str,
}