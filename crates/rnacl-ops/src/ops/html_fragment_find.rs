use std::collections::HashMap;

use rnacl_core::{
    ledger::Ledger,
    operation::{SampleEvolution, TypedOperation},
    sample::{Sample, Trace},
};

use crate::html_selector::{HtmlSelectorError, HtmlSelectorOptions};

pub(crate) struct HtmlFragmentFind;

impl TypedOperation for HtmlFragmentFind {
    type Options = HtmlSelectorOptions;
    type Error = HtmlSelectorError;

    fn description(&self) -> &'static str {
        "Find elements in an HTML fragment."
    }

    fn eval_sample(
        &self,
        _ledger: &Ledger,
        input: Sample,
        options: Self::Options,
    ) -> Result<SampleEvolution, Self::Error> {
        let fragment = scraper::Html::parse_fragment(input.content());
        let selector = options.parse()?;

        Ok(SampleEvolution::Split(
            fragment
                .select(&selector)
                .map(|selection| input.evolve(Trace::new(HashMap::new()), selection.html()))
                .collect(),
        ))
    }
}
