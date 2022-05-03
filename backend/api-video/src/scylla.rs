use once_cell::sync::OnceCell;
use scylla::{Session, SessionBuilder, transport::errors::NewSessionError};

static SESSION: OnceCell<Session> = OnceCell::new();

pub async fn init_scylla() -> Result<(), NewSessionError> {
    let uri = std::env::var("SCYLLA_URI")
        .unwrap_or_else(|_| "video-scylla:9042".to_string());

    let session = SessionBuilder::new()
        .known_node(uri)
        .build()
        .await;
    
        match session {
            Ok(sess) => SESSION.set(sess).map_err(|_| ()).expect("Unable to set OnceCell"),
            Err(e) => return Err(e),
        };
        Ok(())
}

