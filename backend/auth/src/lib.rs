pub mod rpc {
    tonic::include_proto!("auth");
}

pub mod db {
    use msostream::{establish_connection, user};

    pub fn create_user(name: &str, email: &str, password: &str) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        user::create_user(&establish_connection(), &id, name, email, password);
        id
    }
}

pub fn test() {}
