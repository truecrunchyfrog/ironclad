#[derive(thiserror::Error, Debug)]
pub enum OperationError {
    #[error("invalid json value: {0}")]
    Json(#[source] serde_json::Error),

    #[error("invalid operation options: {0}")]
    TomlDe(#[source] toml::de::Error),

    #[error("operation io error: {0}")]
    Io(#[source] std::io::Error),

    #[error("operation execution failed: {0}")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

impl From<serde_json::Error> for OperationError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

impl From<toml::de::Error> for OperationError {
    fn from(value: toml::de::Error) -> Self {
        Self::TomlDe(value)
    }
}

impl From<std::io::Error> for OperationError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
