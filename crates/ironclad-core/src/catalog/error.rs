use std::path::PathBuf;

use crate::{
    fact::{error::FactError, id::FactId},
    recipe::RecipeError,
};

#[derive(thiserror::Error, Debug)]
pub enum CatalogError {
    #[error("no catalog found at {0}")]
    PathNotFound(PathBuf),

    #[error("catalog already initialized at {0}")]
    PathAlreadyExists(PathBuf),

    #[error("path already exists, but is not a directory: {0}")]
    PathNotDirectory(PathBuf),

    #[error("an operation does not exist by such ID: {0}")]
    OperationNotFound(String),

    #[error("dependency fact could not be found: {0}")]
    DependencyFactNotFound(FactId),

    #[error(transparent)]
    Fact(#[from] FactError),

    #[error(transparent)]
    Recipe(#[from] RecipeError),

    #[error(transparent)]
    SystemTime(#[from] std::time::SystemTimeError),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
