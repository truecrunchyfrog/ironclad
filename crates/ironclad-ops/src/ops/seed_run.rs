use std::{collections::HashMap, process::Command};

use ironclad_core::{
    catalog::Catalog,
    operation::TypedOperation,
    sample::{Sample, Trace},
};
use serde::Deserialize;

pub(crate) struct SeedRun;

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub(crate) struct Options {
    program: String,
    args: Vec<String>,
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    FromUtf8(#[from] std::string::FromUtf8Error),
}

impl TypedOperation for SeedRun {
    type Options = Options;
    type Error = Error;

    fn description(&self) -> &'static str {
        "Execute a program."
    }

    fn eval_all(
        &self,
        catalog: &Catalog,
        _input: Vec<Sample>,
        options: Self::Options,
    ) -> Result<Vec<Sample>, Self::Error> {
        let output = Command::new(options.program)
            .current_dir(catalog.container_dir_path())
            .args(options.args)
            .output()?;
        Ok(vec![Sample::new(
            Trace::new(HashMap::new()),
            String::from_utf8(output.stdout)?,
        )])
    }
}
