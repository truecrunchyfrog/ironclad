use std::{collections::HashMap, path::PathBuf};

use glob::glob;
use ironclad_core::{
    operation::{OperationContext, TypedOperation},
    sample::{Sample, Trace},
};
use serde::{Deserialize, Serialize};

pub(crate) struct SeedFileText;

#[derive(Deserialize, Serialize, Clone, Default)]
#[serde(deny_unknown_fields)]
pub(crate) struct Options {
    files: Vec<String>,
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error(transparent)]
    Pattern(#[from] glob::PatternError),

    #[error(transparent)]
    Glob(#[from] glob::GlobError),

    #[error("failed to read file as UTF-8 text: {path}")]
    ReadFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("glob pattern path is not valid utf-8: {path}")]
    NonUtf8GlobPatternPath { path: PathBuf },

    #[error("matched file is outside catalog container: {path}")]
    PathOutsideCatalogContainer { path: PathBuf },
}

impl TypedOperation for SeedFileText {
    type Options = Options;
    type Error = Error;

    fn description(&self) -> &'static str {
        "Read text content from files."
    }

    fn eval_all(
        &self,
        context: &OperationContext,
        _input: Vec<Sample>,
        options: Self::Options,
    ) -> Result<Vec<Sample>, Self::Error> {
        let base_path = context.working_dir();

        let paths = options
            .files
            .iter()
            .map(|pattern| {
                let path = base_path.join(pattern);
                let pattern = path
                    .to_str()
                    .ok_or_else(|| Error::NonUtf8GlobPatternPath { path: path.clone() })?;
                glob(pattern).map_err(Error::from)
            })
            .collect::<Result<Vec<_>, Error>>()?
            .into_iter()
            .flat_map(std::iter::Iterator::collect::<Vec<_>>)
            .collect::<Result<Vec<_>, glob::GlobError>>()?;

        let files = paths
            .into_iter()
            .map(|path| {
                let text = std::fs::read_to_string(&path).map_err(|source| Error::ReadFile {
                    path: path.clone(),
                    source,
                })?;

                Ok(Sample::new(
                    Trace::new(HashMap::from([(
                        String::from("path"),
                        path.strip_prefix(base_path)
                            .map_err(|_| Error::PathOutsideCatalogContainer { path: path.clone() })?
                            .to_string_lossy()
                            .to_string(),
                    )])),
                    text,
                ))
            })
            .collect::<Result<Vec<_>, Error>>()?;

        Ok(files)
    }
}
