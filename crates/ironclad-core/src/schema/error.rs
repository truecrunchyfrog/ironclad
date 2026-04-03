use crate::{operation::OperationError, registry::error::RegistryError};

#[derive(thiserror::Error, Debug)]
pub enum SchemaError {
    #[error("schema is empty")]
    Empty,

    #[error("index {index} is larger than schema's length {length}")]
    OutOfRange { index: usize, length: usize },

    #[error(transparent)]
    Registry(#[from] RegistryError),

    #[error(transparent)]
    Operation(#[from] OperationError),
}
