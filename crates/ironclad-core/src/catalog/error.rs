use std::path::PathBuf;

use crate::{
    fact::{dependencies::SortDependenciesError, error::FactError},
    recipe::RecipeError,
};

#[derive(thiserror::Error, Debug)]
pub enum CatalogError {
    #[error("catalog not found at {0}")]
    PathNotFound(PathBuf),

    #[error("catalog already exists at {0}")]
    PathAlreadyExists(PathBuf),

    #[error("catalog path is not a directory: {0}")]
    PathNotDirectory(PathBuf),

    #[error("fact label not found in catalog index: {0}")]
    LabelNotInIndex(String),

    #[error("fact id not found in catalog index: {0}")]
    IdNotInIndex(String),

    #[error("fact selector not found: {0}")]
    FactNotFound(String),

    #[error("fact import not found: {0}")]
    ImportNotFound(String),

    #[error("duplicate export key '{key}' declared by facts: {fact_labels:?}")]
    DuplicateExportKey {
        key: String,
        fact_labels: Vec<String>,
    },

    #[error(
        "sample with trace {trace_key}={trace_value} to export as '{export_key}' not found when evaluating batch for fact {fact_label}"
    )]
    SampleToExportNotFound {
        fact_label: String,
        export_key: String,
        trace_key: String,
        trace_value: String,
    },

    #[error("operation not registered: {0}")]
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
    TomlSer(#[from] toml::ser::Error),

    #[error(transparent)]
    TomlDe(#[from] toml::de::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
