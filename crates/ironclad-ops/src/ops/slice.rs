use std::ops::Range;

use ironclad_core::{
    operation::{OperationContext, TypedOperation},
    sample::Sample,
};
use serde::{Deserialize, Serialize};

pub(crate) struct Slice;

#[derive(Deserialize, Serialize, Clone, Default)]
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
        "Take a slice of the current batch.\n\n\
This operation runs over the whole input batch rather than per sample. It can \
drop a number of samples from the front and optionally keep only a limited \
number after that. If the requested range is out of bounds, the result is an \
empty batch rather than an error.\n\n\
Options:\n\
- `drop`: number of samples to skip from the start.\n\
- `take`: number of samples to keep after dropping. If omitted, the rest of the \
  batch is kept."
    }

    fn eval_all(
        &self,
        _context: &OperationContext,
        input: Vec<Sample>,
        options: Self::Options,
    ) -> Result<Vec<Sample>, Self::Error> {
        let range = Range {
            start: options.drop.unwrap_or(0),
            end: options
                .take
                .map_or_else(|| input.len(), |take| options.drop.unwrap_or(0) + take),
        };

        Ok(input.get(range).unwrap_or_default().to_vec())
    }
}
