use crate::{operation::OperationError, registry::error::RegistryError};

#[derive(thiserror::Error, Debug)]
pub enum RecipeError {
    #[error(transparent)]
    Registry(#[from] RegistryError),

    #[error("operation failed: {operation_id}: {source}")]
    Operation {
        operation_id: String,
        source: OperationError,
    },
}
