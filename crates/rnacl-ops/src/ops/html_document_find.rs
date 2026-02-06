use std::collections::HashMap;

use rnacl_core::{
    ledger::Ledger,
    operation::{SampleEvolution, TypedOperation},
    sample::{Sample, Trace},
};

use crate::html_selector::{HtmlSelectorError, HtmlSelectorOptions};

pub(crate) struct HtmlDocumentFind;

impl TypedOperation for HtmlDocumentFind {
    type Options = HtmlSelectorOptions;
    type Error = HtmlSelectorError;

    fn description(&self) -> &'static str {
        "Find elements in an HTML document."
    }

    fn eval_sample(
        &self,
        _ledger: &Ledger,
        input: Sample,
        options: Self::Options,
    ) -> Result<SampleEvolution, Self::Error> {
        let document = scraper::Html::parse_document(input.content());
        let selector = options.parse()?;

        Ok(SampleEvolution::Split(
            document
                .select(&selector)
                .map(|selection| input.evolve(Trace::new(HashMap::new()), selection.html()))
                .collect(),
        ))
    }
}
