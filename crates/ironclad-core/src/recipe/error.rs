use crate::{operation::OperationError, registry::error::RegistryError};

#[derive(thiserror::Error, Debug)]
pub enum RecipeError {
    #[error("recipe is empty")]
    Empty,

    #[error("index {index} is larger than recipe's length {length}")]
    OutOfRange { index: usize, length: usize },

    #[error(transparent)]
    Registry(#[from] RegistryError),

    #[error(transparent)]
    Operation(#[from] OperationError),
}
