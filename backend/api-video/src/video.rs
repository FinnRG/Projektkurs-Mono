use crate::videos::v1::Video;

impl Video {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Unable to deserialize video")
    }
}

impl From<&str> for Video {
    fn from(item: &str) -> Video {
        serde_json::from_str(item).expect("Unable to serialize video")
    }
}

impl From<String> for Video {
    fn from(item: String) -> Video {
        serde_json::from_str(&item).expect("Unable to serialize video")
    }
}
