use std::collections::HashMap;

use ironclad_core::{
    operation::{OperationContext, TypedOperation},
    sample::{Sample, Trace},
};
use serde::{Deserialize, Serialize};

use crate::text_selector::TextSelector;

pub(crate) struct TextFind;

#[derive(Deserialize, Serialize, Clone, Default)]
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
        "Find text matches inside each sample.\n\n\
This operation supports either plain substring matching or regular expressions. \
Regex syntax follows the Rust `regex` crate:\n\
https://docs.rs/regex/latest/regex/#syntax\n\n\
Each match becomes one output sample. The operation records `start` and `end` \
trace entries for the matched byte range. When regex mode is used, `expand` can \
build the output content from capture groups.\n\n\
Options:\n\
- `text`: plain substring to match.\n\
- `regex`: regular expression to match.\n\
- `expand`: optional regex expansion template used instead of the full match.\n\n\
Exactly one of `text` or `regex` should be provided."
    }

    fn eval_each(
        &self,
        _context: &OperationContext,
        input: Sample,
        options: Self::Options,
    ) -> Result<Vec<Sample>, Self::Error> {
        Ok(match options.selection {
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
        })
    }
}
