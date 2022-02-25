use crate::*;
use bcrypt::{hash, verify, DEFAULT_COST};
use models::{NewUser, Notification, Subscription, User};

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

pub fn get_subscriptions(conn: &PgConnection, user_id: &str) -> Vec<Subscription> {
    use schema::subscriptions;

    subscriptions::table
        .filter(subscriptions::user_id.eq(user_id))
        .get_results(conn)
        .expect("Unable to fetch subscriptions for user")
}

pub fn get_notifications(conn: &PgConnection, user_id: &str) -> Vec<Notification> {
    use schema::notifications;

    notifications::table
        .filter(notifications::user_id.eq(user_id))
        .get_results(conn)
        .expect("Unable to fetchg notifications for user")
}

pub fn remove_notification(conn: &PgConnection, user_id: &str, tag_id: i32, video_id: &str) {
    use schema::notifications;

    diesel::delete(notifications::table.find((user_id, tag_id, video_id)))
        .execute(conn)
        .expect("Unable to delete notification");
}
