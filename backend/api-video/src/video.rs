use crate::videos::v1::Status as VideoStatus;

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
