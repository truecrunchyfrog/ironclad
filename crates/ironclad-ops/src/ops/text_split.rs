use std::collections::HashMap;

use ironclad_core::{
    catalog::Catalog,
    operation::{SampleEvolution, TypedOperation},
    sample::{Sample, Trace},
};
use serde::Deserialize;

pub(crate) struct TextSplit;

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub(crate) enum Options {
    #[serde(rename = "at_index")]
    AtIndex(usize),
    #[serde(rename = "on_text")]
    OnText { text: String, max: Option<usize> },
    #[serde(rename = "on_text_inclusive")]
    OnTextInclusive { text: String },
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {}

impl TypedOperation for TextSplit {
    type Options = Options;
    type Error = Error;

    fn description(&self) -> &'static str {
        "Split text into samples."
    }

    fn eval_sample(
        &self,
        _catalog: &Catalog,
        input: Sample,
        options: Self::Options,
    ) -> Result<SampleEvolution, Self::Error> {
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

        Ok(SampleEvolution::Split(
            parts
                .into_iter()
                .map(|content| input.evolve(Trace::new(HashMap::new()), content.to_string()))
                .collect::<Vec<_>>(),
        ))
    }
}
