use super::schema::users;
use crate::User;
use diesel::Queryable;

#[derive(Queryable, Insertable, AsChangeset)]
#[table_name = "users"]
pub struct DBUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
}

impl From<User> for DBUser {
    fn from(item: User) -> Self {
        DBUser {
            id: item.id,
            name: item.name,
            email: item.email,
            password: item.password,
        }
    }
}

impl From<DBUser> for User {
    fn from(item: DBUser) -> Self {
        User {
            id: item.id,
            name: item.name,
            email: item.email,
            password: item.password,
        }
    }
}
