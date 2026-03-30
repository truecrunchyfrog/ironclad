use std::path::PathBuf;

use crate::{
    cell::{error::CellError, id::CellId},
    pipeline::PipelineError,
};

#[derive(thiserror::Error, Debug)]
pub enum LedgerError {
    #[error("no ledger found at {0}")]
    PathNotFound(PathBuf),

    #[error("ledger already initialized at {0}")]
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
    Pipeline(#[from] PipelineError),

    #[error(transparent)]
    SystemTime(#[from] std::time::SystemTimeError),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
