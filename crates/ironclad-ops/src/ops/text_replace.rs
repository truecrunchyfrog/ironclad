use std::collections::HashMap;

use ironclad_core::{
    operation::{OperationContext, TypedOperation},
    sample::{Sample, Trace},
};
use serde::{Deserialize, Serialize};

use crate::text_selector::TextSelector;

pub(crate) struct TextReplace;

#[derive(Deserialize, Serialize, Clone, Default)]
#[serde(deny_unknown_fields)]
pub(crate) struct Options {
    #[serde(flatten)]
    selection: TextSelector,
    replacement: String,
    max: Option<usize>,
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {}

impl TypedOperation for TextReplace {
    type Options = Options;
    type Error = Error;

    fn description(&self) -> &'static str {
        "Replace text inside each sample.\n\n\
This operation supports either plain substring replacement or regular \
expression replacement. Regex syntax follows the Rust `regex` crate:\n\
https://docs.rs/regex/latest/regex/#syntax\n\n\
The operation returns one output sample for each input sample. Replacement is \
global by default; `max` can limit the number of replacements.\n\n\
Options:\n\
- `text`: plain substring to replace.\n\
- `regex`: regular expression to replace.\n\
- `replacement`: replacement string.\n\
- `max`: optional maximum number of replacements.\n\n\
Exactly one of `text` or `regex` should be provided."
    }

    fn eval_each(
        &self,
        _context: &OperationContext,
        input: Sample,
        options: Self::Options,
    ) -> Result<Vec<Sample>, Self::Error> {
        Ok(vec![
            input.evolve(
                Trace::new(HashMap::new()),
                match options {
                    Self::Options {
                        selection: TextSelector::Plaintext(plaintext),
                        replacement,
                        max: None,
                    } => input.content().replace(&plaintext, &replacement),

                    Self::Options {
                        selection: TextSelector::Plaintext(plaintext),
                        replacement,
                        max: Some(max),
                    } => input.content().replacen(&plaintext, &replacement, max),

                    Self::Options {
                        selection: TextSelector::Regex(regex),
                        replacement,
                        max: None,
                    } => regex.replace_all(input.content(), replacement).to_string(),

                    Self::Options {
                        selection: TextSelector::Regex(regex),
                        replacement,
                        max: Some(max),
                    } => regex
                        .replacen(input.content(), max, replacement)
                        .to_string(),
                },
            ),
        ])
    }
}
