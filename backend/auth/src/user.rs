use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Unable to serialize user struct")
    }
}

impl User {
    pub fn from_json(item: &str) -> Self {
        serde_json::from_str(item).expect("Unable to deserialize string into user struct")
    }
}
