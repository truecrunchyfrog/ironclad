use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum CellError {
    #[error("cell already exists at {0}")]
    PathAlreadyExists(PathBuf),

    #[error("cell not found at {0}")]
    PathNotFound(PathBuf),

    #[error("cell not found: {0}")]
    NoSuchCellId(String),

    #[error("ambiguous cell ID: {0}")]
    AmbiguousCellId(String),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
