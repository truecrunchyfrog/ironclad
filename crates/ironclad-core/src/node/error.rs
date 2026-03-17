use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum NodeError {
    #[error("node already exists at {0}")]
    PathAlreadyExists(PathBuf),

    #[error("node not found at {0}")]
    PathNotFound(PathBuf),

    #[error("node not found: {0}")]
    NoSuchNodeId(String),

    #[error("ambiguous node ID: {0}")]
    AmbiguousNodeId(String),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
