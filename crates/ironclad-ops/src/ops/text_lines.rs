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
        "Split lines into samples."
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
