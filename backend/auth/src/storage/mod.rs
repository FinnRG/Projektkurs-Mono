pub mod models;
pub mod schema;

use crate::diesel::ExpressionMethods;
use crate::storage::models::DBUser;
use crate::DATABASE_URL;
use diesel::{Connection, PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use self::models::NewDBUser;

pub struct Store {
    conn: PgConnection,
}

impl Store {
    pub fn new() -> Self {
        let conn = PgConnection::establish(&DATABASE_URL).expect("Unable to connect to PostgreSQL");
        Store { conn }
    }

    pub fn get_user(&mut self, email: &str) -> DBUser {
        use schema::users;

        users::table
            .filter(users::email.eq(email))
            .first::<DBUser>(&self.conn)
            .expect("Error loading users")
    }

    // TODO: Get user email with id and THEN delete password with associated email
    pub fn del_user(&mut self, id: &Uuid) {
        use schema::users;

        diesel::delete(users::table.filter(users::id.eq(id)))
            .execute(&self.conn)
            .expect("Unable to delete user");
    }

    pub fn create_user(&mut self, user: &NewDBUser) -> Result<Uuid, diesel::result::Error> {
        use schema::users;

        diesel::insert_into(users::table)
            .values(user)
            .returning(users::id)
            .get_result::<Uuid>(&self.conn)
    }
}
