use std::ops::Range;

use ironclad_core::{ledger::Ledger, operation::TypedOperation, sample::Sample};
use serde::Deserialize;

pub(crate) struct Slice;

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub(crate) struct Options {
    drop: Option<usize>,
    take: Option<usize>,
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {}

impl TypedOperation for Slice {
    type Options = Options;
    type Error = Error;

    fn description(&self) -> &'static str {
        "Select a subslice of the samples."
    }

    fn eval_sample_set(
        &self,
        _ledger: &Ledger,
        input: Vec<Sample>,
        options: Self::Options,
    ) -> Result<Vec<Vec<Sample>>, Self::Error> {
        let range = Range {
            start: options.drop.unwrap_or(0),
            end: options
                .take
                .map_or_else(|| input.len(), |take| options.drop.unwrap_or(0) + take),
        };

        Ok(vec![input.get(range).unwrap_or_default().to_vec()])
    }
}
