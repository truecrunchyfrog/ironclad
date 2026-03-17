use std::{collections::HashMap, fs};

use glob::glob;
use rnacl_core::{
    ledger::Ledger,
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

    fn eval(
        &self,
        ledger: &Ledger,
        _input: Vec<Vec<Sample>>,
        options: Self::Options,
    ) -> Result<Vec<Vec<Sample>>, Self::Error> {
        let base_path = ledger.container_dir();

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
            .flat_map(|p| p.collect::<Vec<_>>())
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
                                    .expect("path is not relative to ledger container")
                                    .to_string_lossy()
                                    .to_string(),
                            )])),
                            text,
                        )
                    })
                    .ok()
            })
            .collect::<Vec<_>>();

        Ok(vec![files])
    }
}
