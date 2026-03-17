use std::collections::HashMap;

use ironclad_core::{
    ledger::Ledger,
    operation::{SampleEvolution, TypedOperation},
    sample::{Sample, Trace},
};
use serde::Deserialize;

pub(crate) struct HtmlFind;

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub(crate) struct Options {
    selector: String,
    #[serde(default)]
    document: bool,
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("invalid CSS selector: {0}")]
    Selector(String),
}

impl TypedOperation for HtmlFind {
    type Options = Options;
    type Error = Error;

    fn description(&self) -> &'static str {
        "Find elements in HTML."
    }

    fn eval_sample(
        &self,
        _ledger: &Ledger,
        input: Sample,
        options: Self::Options,
    ) -> Result<SampleEvolution, Self::Error> {
        let fragment = if options.document {
            scraper::Html::parse_document
        } else {
            scraper::Html::parse_fragment
        }(input.content());
        let selector = scraper::Selector::parse(&options.selector)
            .map_err(|err| Error::Selector(err.to_string()))?;

        Ok(SampleEvolution::Split(
            fragment
                .select(&selector)
                .map(|selection| input.evolve(Trace::new(HashMap::new()), selection.html()))
                .collect(),
        ))
    }
}
