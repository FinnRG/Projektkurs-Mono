use super::schema::users;
use crate::User;
use diesel::Queryable;
use uuid::Uuid;

#[derive(Queryable, AsChangeset)]
#[table_name = "users"]
pub struct DBUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewDBUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

impl From<User> for DBUser {
    fn from(item: User) -> Self {
        DBUser {
            id: Uuid::parse_str(&item.id).expect("Unable to convert string to uuid"),
            name: item.name,
            email: item.email,
            password: item.password,
        }
    }
}

impl From<DBUser> for User {
    fn from(item: DBUser) -> Self {
        User {
            id: item.id.to_string(),
            name: item.name,
            email: item.email,
            password: item.password,
        }
    }
}
