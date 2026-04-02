use std::collections::HashMap;

use ironclad_core::{
    cluster::Cluster,
    operation::{SampleEvolution, TypedOperation},
    sample::{Sample, Trace},
};
use serde::Deserialize;

use crate::text_selector::TextSelector;

pub(crate) struct TextFind;

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub(crate) struct Options {
    #[serde(flatten)]
    selection: TextSelector,
    expand: Option<String>,
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {}

impl TypedOperation for TextFind {
    type Options = Options;
    type Error = Error;

    fn description(&self) -> &'static str {
        "Match text. https://docs.rs/regex/latest/regex/#syntax"
    }

    fn eval_sample(
        &self,
        _cluster: &Cluster,
        input: Sample,
        options: Self::Options,
    ) -> Result<SampleEvolution, Self::Error> {
        Ok(SampleEvolution::Split(match options.selection {
            TextSelector::Plaintext(plaintext) => input
                .content()
                .match_indices(&plaintext)
                .map(|(start, value)| {
                    input.evolve(
                        Trace::new(HashMap::from([
                            ("start".to_string(), start.to_string()),
                            ("end".to_string(), (start + value.len()).to_string()),
                        ])),
                        value.to_string(),
                    )
                })
                .collect(),
            TextSelector::Regex(regex) => regex
                .captures_iter(input.content())
                .map(|captures| {
                    let captures_match = captures.get_match();
                    input.evolve(
                        Trace::new(HashMap::from([
                            ("start".to_string(), captures_match.start().to_string()),
                            ("end".to_string(), captures_match.end().to_string()),
                        ])),
                        options.expand.as_ref().map_or_else(
                            || captures_match.as_str().to_string(),
                            |expand| {
                                let mut buf = String::new();
                                captures.expand(expand, &mut buf);
                                buf
                            },
                        ),
                    )
                })
                .collect::<Vec<_>>(),
        }))
    }
}
