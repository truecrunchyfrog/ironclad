use std::collections::HashMap;

use ironclad_core::{
    operation::{OperationContext, TypedOperation},
    sample::{Sample, Trace},
};
use serde::{Deserialize, Serialize};

pub(crate) struct TextSplit;

#[derive(Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub(crate) enum Options {
    #[serde(rename = "at_index")]
    AtIndex(usize),
    #[serde(rename = "on_text")]
    OnText { text: String, max: Option<usize> },
    #[serde(rename = "on_text_inclusive")]
    OnTextInclusive { text: String },
}

impl Default for Options {
    fn default() -> Self {
        Self::AtIndex(0)
    }
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {}

impl TypedOperation for TextSplit {
    type Options = Options;
    type Error = Error;

    fn description(&self) -> &'static str {
        "Split each sample into multiple samples.\n\n\
This operation supports three modes: split at a byte index, split on a text \
delimiter, or split inclusively while keeping the delimiter on each piece.\n\n\
Each produced piece becomes one output sample. When the requested byte index is \
invalid or out of bounds, the result is an empty output for that input sample.\n\n\
Options:\n\
- `at_index = N`: split into two pieces at byte index `N`.\n\
- `on_text = { text = \"...\", max = N? }`: split on a delimiter, optionally \
  limiting the number of pieces.\n\
- `on_text_inclusive = { text = \"...\" }`: split while keeping the delimiter \
  attached to each returned piece."
    }

    fn eval_each(
        &self,
        _context: &OperationContext,
        input: Sample,
        options: Self::Options,
    ) -> Result<Vec<Sample>, Self::Error> {
        let parts = match options {
            Self::Options::AtIndex(mid) => input
                .content()
                .split_at_checked(mid)
                .map_or_else(Vec::new, |(fst, snd)| vec![fst, snd]),
            Options::OnText { text, max: None } => input.content().split(&text).collect(),
            Options::OnText {
                text,
                max: Some(max),
            } => input.content().splitn(max, &text).collect(),
            Options::OnTextInclusive { text } => input.content().split_inclusive(&text).collect(),
        };

        Ok(parts
            .into_iter()
            .map(|content| input.evolve(Trace::new(HashMap::new()), content.to_string()))
            .collect::<Vec<_>>())
    }
}
