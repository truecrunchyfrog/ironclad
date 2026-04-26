#[derive(thiserror::Error, Debug)]
pub enum RegistryError {
    #[error("operation already registered: {0}")]
    OperationAlreadyExists(String),

    #[error("operation not registered: {0}")]
    OperationNotFound(String),
}
