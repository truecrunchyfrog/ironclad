use std::{
    collections::HashMap,
    io::Write,
    process::{Command, Stdio},
};

use ironclad_core::{
    operation::{OperationContext, TypedOperation},
    sample::{Sample, Trace},
};
use serde::{Deserialize, Serialize};

pub(crate) struct Run;

#[derive(Deserialize, Serialize, Clone, Default)]
#[serde(deny_unknown_fields)]
pub(crate) struct Options {
    program: String,
    #[serde(default)]
    args: Vec<String>,
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    FromUtf8(#[from] std::string::FromUtf8Error),

    #[error("unable to open stdin pipe for command: {program}")]
    StdinUnavailable { program: String },

    #[error("command exited unsuccessfully: {program} {code}: {stderr}")]
    ExitStatus {
        program: String,
        code: String,
        stderr: String,
    },
}

impl TypedOperation for Run {
    type Options = Options;
    type Error = Error;

    fn description(&self) -> &'static str {
        "Execute a program for each sample, piping sample content to stdin."
    }

    fn eval_each(
        &self,
        context: &OperationContext,
        input: Sample,
        options: Self::Options,
    ) -> Result<Vec<Sample>, Self::Error> {
        let program = options.program;
        let mut child = Command::new(&program)
            .current_dir(context.working_dir())
            .args(options.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        child
            .stdin
            .take()
            .ok_or_else(|| Error::StdinUnavailable {
                program: program.clone(),
            })?
            .write_all(input.content().as_bytes())?;

        let output = child.wait_with_output()?;

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

        Ok(vec![input.evolve(
            Trace::new(HashMap::new()),
            String::from_utf8(output.stdout)?,
        )])
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use ironclad_core::{
        operation::{OperationContext, TypedOperation},
        sample::{Sample, Trace},
    };

    use super::{Options, Run};

    #[test]
    fn pipes_each_sample_to_command_stdin() {
        let op = Run;
        let context = OperationContext::for_working_dir(std::env::temp_dir());
        let input = Sample::new(Trace::new(HashMap::new()), String::from("hello"));

        let output = op
            .eval_all(
                &context,
                vec![input],
                Options {
                    program: String::from("cat"),
                    args: Vec::new(),
                },
            )
            .expect("run op");

        assert_eq!(output.len(), 1);
        assert_eq!(output[0].content(), "hello");
    }
}
