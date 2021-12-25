use crate::schema::users;
use crate::schema::videos;
use std::time::SystemTime;

#[derive(Queryable, Identifiable, Debug)]
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

pub struct Video<'a> {
    pub id: &'a str,
    pub user_id: &'a str,
    pub title: &'a str,
    pub description: &'a str,
    pub creation_date: SystemTime,
}

#[derive(Insertable, Identifiable, Associations)]
#[belongs_to(User)]
#[table_name="videos"]
pub struct NewVideo<'a> {
    pub id: &'a str,
    pub user_id: &'a str,
    pub title: &'a str,
    pub description: &'a str,
}