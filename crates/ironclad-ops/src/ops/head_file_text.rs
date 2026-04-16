use std::{collections::HashMap, fs};

use glob::glob;
use ironclad_core::{
    catalog::Catalog,
    operation::TypedOperation,
    sample::{Sample, Trace},
};
use serde::Deserialize;

pub(crate) struct HeadFileText;

#[derive(Deserialize, Clone)]
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
}

impl TypedOperation for HeadFileText {
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
            .collect::<Result<Vec<_>, _>>()?;

        let files = paths
            .into_iter()
            .filter_map(|path| {
                fs::read_to_string(&path)
                    .map(|text| {
                        Sample::new(
                            Trace::new(HashMap::from([(
                                String::from("path"),
                                path.strip_prefix(&base_path)
                                    .expect("path is not relative to catalog container")
                                    .to_string_lossy()
                                    .to_string(),
                            )])),
                            text,
                        )
                    })
                    .ok()
            })
            .collect::<Vec<_>>();

        Ok(files)
    }
}
