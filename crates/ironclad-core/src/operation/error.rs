#[derive(thiserror::Error, Debug)]
pub enum OperationError {
    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    TomlDe(#[from] toml::de::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("operation: {0}")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}
