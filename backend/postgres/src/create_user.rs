use crate::*;
use models::{User, NewUser};
use diesel::prelude;

pub fn create_user<'a>(conn: &PgConnection, name: &'a str, email: &'a str, password: &'a str) -> User {
    use schema::users;
    
    let connection = establish_connection();

    let new_user = NewUser {
        name: name,
        email: email,
        password: password,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new user")
}