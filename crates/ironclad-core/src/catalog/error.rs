use std::path::PathBuf;

use crate::{
    fact::{dependencies::SortDependenciesError, error::FactError},
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

    #[error("fact label not in index: {0}")]
    LabelNotInIndex(String),

    #[error("fact ID not in index: {0}")]
    IdNotInIndex(String),

    #[error("import not found: {0}")]
    ImportNotFound(String),

    #[error(
        "sample with trace {trace_key}={trace_value} to export as '{export_key}' not found when evaluating batch for fact {fact_label}"
    )]
    SampleToExportNotFound {
        fact_label: String,
        export_key: String,
        trace_key: String,
        trace_value: String,
    },

    #[error("an operation does not exist by such ID: {0}")]
    OperationNotFound(String),

    #[error("unable to sort dependencies: {0}")]
    SortDependenciesError(#[from] SortDependenciesError),

    #[error(transparent)]
    StripPrefix(#[from] std::path::StripPrefixError),

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
