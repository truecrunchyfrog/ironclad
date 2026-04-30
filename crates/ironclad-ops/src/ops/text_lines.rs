use std::collections::HashMap;

use ironclad_core::{
    operation::{OperationContext, TypedOperation},
    sample::{Sample, Trace},
};

pub(crate) struct TextLines;

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {}

impl TypedOperation for TextLines {
    type Options = ();
    type Error = Error;

    fn description(&self) -> &'static str {
        "Split each sample into one sample per line.\n\n\
This operation uses Rust string line splitting semantics, equivalent to \
`str::lines()`:\n\
https://doc.rust-lang.org/std/primitive.str.html#method.lines\n\n\
Each line becomes a new sample. The original sample trace is preserved by \
appending an empty trace step.\n\n\
Options: none."
    }

    fn eval_each(
        &self,
        _context: &OperationContext,
        input: Sample,
        _options: Self::Options,
    ) -> Result<Vec<Sample>, Self::Error> {
        Ok(input
            .content()
            .lines()
            .map(|line| input.evolve(Trace::new(HashMap::new()), line.to_string()))
            .collect::<Vec<_>>())
    }
}
