#[derive(thiserror::Error, Debug)]
pub enum RegistryError {
    #[error("operation with same ID already exists: {0}")]
    OperationAlreadyExists(String),

    #[error("operation not found in registry: {0}")]
    OperationNotFound(String),
}
