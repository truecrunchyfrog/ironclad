use std::collections::HashMap;

use ironclad_core::{
    cluster::Cluster,
    operation::{SampleEvolution, TypedOperation},
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

    fn eval_sample(
        &self,
        _cluster: &Cluster,
        input: Sample,
        _options: Self::Options,
    ) -> Result<SampleEvolution, Self::Error> {
        Ok(SampleEvolution::Split(
            input
                .content()
                .lines()
                .map(|line| input.evolve(Trace::new(HashMap::new()), line.to_string()))
                .collect::<Vec<_>>(),
        ))
    }
}
