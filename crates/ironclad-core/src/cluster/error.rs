use std::path::PathBuf;

use crate::{
    cell::{error::CellError, id::CellId},
    schema::SchemaError,
};

#[derive(thiserror::Error, Debug)]
pub enum ClusterError {
    #[error("no cluster found at {0}")]
    PathNotFound(PathBuf),

    #[error("cluster already initialized at {0}")]
    PathAlreadyExists(PathBuf),

    #[error("path already exists, but is not a directory: {0}")]
    PathNotDirectory(PathBuf),

    #[error("an operation does not exist by such ID: {0}")]
    OperationNotFound(String),

    #[error("dependency cell could not be found: {0}")]
    DependencyCellNotFound(CellId),

    #[error(transparent)]
    Cell(#[from] CellError),

    #[error(transparent)]
    Schema(#[from] SchemaError),

    #[error(transparent)]
    SystemTime(#[from] std::time::SystemTimeError),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
