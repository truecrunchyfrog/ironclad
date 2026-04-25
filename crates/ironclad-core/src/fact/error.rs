use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum FactError {
    #[error("fact already exists: {0}")]
    PathAlreadyExists(PathBuf),

    #[error("fact not found: {0}")]
    PathNotFound(PathBuf),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    TomlDe(#[from] toml::de::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
