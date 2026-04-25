use std::collections::HashMap;

use ironclad_core::{
    catalog::Catalog,
    operation::TypedOperation,
    sample::{Sample, Trace},
};
use serde::Deserialize;

use crate::text_selector::TextSelector;

pub(crate) struct TextReplace;

#[derive(Deserialize, Clone, Default)]
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
        "Replace text."
    }

    fn eval_each(
        &self,
        _catalog: &Catalog,
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
