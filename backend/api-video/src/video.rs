use crate::videos::v1::{Status as VideoStatus, Video};

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

impl From<&str> for VideoStatus {
    fn from(item: &str) -> VideoStatus {
        match item {
            "STATUS_FINISHED" => VideoStatus::Finished,
            "STATUS_UPLOADED" => VideoStatus::Uploaded,
            "STATUS_PROCESSED" => VideoStatus::Processed,
            "STATUS_DRAFT" => VideoStatus::Draft,
            _ => VideoStatus::Unspecified,
        }
    }
}

impl VideoStatus {
    #[deprecated]
    pub fn from_str(str: &str) -> VideoStatus {
        match str {
            "STATUS_FINISHED" => VideoStatus::Finished,
            "STATUS_UPLOADED" => VideoStatus::Uploaded,
            "STATUS_PROCESSED" => VideoStatus::Processed,
            "STATUS_DRAFT" => VideoStatus::Draft,
            _ => VideoStatus::Unspecified,
        }
    }
}
