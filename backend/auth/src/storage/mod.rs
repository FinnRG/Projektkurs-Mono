pub mod schema;
pub mod models;

use crate::DATABASE_URL;
use crate::storage::models::DBUser;
use crate::{user::User};
use diesel::pg::upsert::on_constraint;
use diesel::{PgConnection, Connection, QueryDsl, RunQueryDsl};
use crate::diesel::ExpressionMethods;

pub struct Store {
    conn: PgConnection,
}

impl Store {
    pub fn new() -> Self {
        let conn = PgConnection::establish(&DATABASE_URL).expect("Unable to connect to PostgreSQL");
        Store {
            conn,
        }
    }

    pub fn get_user(&mut self, email: &str) -> DBUser {
        use schema::users;

        users::table
            .filter(users::email.eq(email))
            .first::<DBUser>(&self.conn)
            .expect("Error loading users")
    }

    // TODO: Get user email with id and THEN delete password with associated email
    pub fn del_user(&mut self, email: &str) {
        use schema::users;

        diesel::delete(users::table.filter(users::email.eq(email)))
            .execute(&self.conn)
            .expect("Unable to delete user");
    }

    pub fn set_user(&mut self, user: User) {
        use schema::users;

        let dbuser = models::DBUser::from(user);

        diesel::insert_into(users::table)
            .values(&dbuser)
            .on_conflict(on_constraint("users_pkey"))
            .do_update()
            .set(&dbuser)
            .execute(&self.conn)
            .expect("Failed to set new user");
    }
}
