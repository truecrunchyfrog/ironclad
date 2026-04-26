use std::{collections::HashMap, process::Command};

use ironclad_core::{
    operation::{OperationContext, TypedOperation},
    sample::{Sample, Trace},
};
use serde::{Deserialize, Serialize};

pub(crate) struct SeedRun;

#[derive(Deserialize, Serialize, Clone, Default)]
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

    #[error("command exited unsuccessfully: {program} {code}: {stderr}")]
    ExitStatus {
        program: String,
        code: String,
        stderr: String,
    },
}

impl TypedOperation for SeedRun {
    type Options = Options;
    type Error = Error;

    fn description(&self) -> &'static str {
        "Execute a program."
    }

    fn eval_all(
        &self,
        context: &OperationContext,
        _input: Vec<Sample>,
        options: Self::Options,
    ) -> Result<Vec<Sample>, Self::Error> {
        let program = options.program;
        let output = Command::new(&program)
            .current_dir(context.working_dir())
            .args(options.args)
            .output()?;

        if !output.status.success() {
            return Err(Error::ExitStatus {
                program,
                code: output.status.code().map_or_else(
                    || "terminated by signal".to_string(),
                    |code| code.to_string(),
                ),
                stderr: String::from_utf8_lossy(&output.stderr).trim().to_string(),
            });
        }

        Ok(vec![Sample::new(
            Trace::new(HashMap::new()),
            String::from_utf8(output.stdout)?,
        )])
    }
}
