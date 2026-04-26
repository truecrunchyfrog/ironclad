use std::{collections::HashMap, path::PathBuf};

use glob::glob;
use ironclad_core::{
    catalog::Catalog,
    operation::TypedOperation,
    sample::{Sample, Trace},
};
use serde::Deserialize;

pub(crate) struct SeedFileText;

#[derive(Deserialize, Clone, Default)]
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
}

impl TypedOperation for SeedFileText {
    type Options = Options;
    type Error = Error;

    fn description(&self) -> &'static str {
        "Read text content from files."
    }

    fn eval_all(
        &self,
        catalog: &Catalog,
        _input: Vec<Sample>,
        options: Self::Options,
    ) -> Result<Vec<Sample>, Self::Error> {
        let base_path = catalog.container_dir_path();

        let paths = options
            .files
            .iter()
            .map(|pattern| {
                glob(
                    base_path
                        .join(pattern)
                        .to_str()
                        .expect("glob pattern does not appear to be UTF-8"),
                )
            })
            .collect::<Result<Vec<_>, _>>()?
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
                        path.strip_prefix(&base_path)
                            .expect("path is not relative to catalog container")
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
