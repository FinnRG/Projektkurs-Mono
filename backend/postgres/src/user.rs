use crate::*;
use bcrypt::{hash, verify, DEFAULT_COST};
use models::{NewUser, User};

pub fn create_user<'a>(
    conn: &PgConnection,
    id: &'a str,
    name: &'a str,
    email: &'a str,
    password: &'a str,
) -> User {
    use schema::users;
    let hash = hash(&password, DEFAULT_COST).expect("Unable to hash");

    let new_user = NewUser {
        id,
        name,
        email,
        password: &hash,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new user")
}

pub fn check_password<'a>(
    conn: &PgConnection,
    email: &'a str,
    password: &'a str,
) -> Option<String> {
    use schema::users;

    match users::table
        .filter(users::email.eq(email))
        .get_result::<User>(conn)
    {
        Ok(user) => match verify(password, &user.password) {
            Ok(_) => Some(user.id),
            Err(_) => None,
        },
        Err(_) => None,
    }
}
