use crate::*;
use diesel::pg::upsert::*;
use models::NewSubscription;

pub fn new_subscription<'a>(conn: &PgConnection, user_id: &'a str, tag_id: i32) {
    use schema::subscriptions;

    let new_subscription = NewSubscription { user_id, tag_id };

    diesel::insert_into(subscriptions::table)
        .values(&new_subscription)
        .on_conflict(on_constraint("subscriptions_pkey"))
        .do_nothing()
        .execute(conn)
        .expect("Error creating new subscription");
}

pub fn delete_subscription(conn: &PgConnection, user_id: &str, tag_id: i32) {
    use schema::subscriptions;

    diesel::delete(subscriptions::table.find((user_id, tag_id)))
        .execute(conn)
        .expect("Unable to remove subscription for user and tag");
}

pub fn remove_notification(conn: &PgConnection, user_id: &str, tag_id: i32, video_id: &str) {
    use schema::notifications;

    diesel::delete(notifications::table.find((user_id, tag_id, video_id)))
        .execute(conn)
        .expect("Unable to delete notification");
}
