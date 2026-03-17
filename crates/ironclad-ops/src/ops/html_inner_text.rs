use std::collections::HashMap;

use ironclad_core::{
    ledger::Ledger,
    operation::{SampleEvolution, TypedOperation},
    sample::{Sample, Trace},
};
use scraper::Element;

use crate::fragment_error::FragmentError;

pub(crate) struct HtmlInnerText;

impl TypedOperation for HtmlInnerText {
    type Options = ();
    type Error = FragmentError;

    fn description(&self) -> &'static str {
        "Select the inner text of an HTML element."
    }

    fn eval_sample(
        &self,
        _ledger: &Ledger,
        input: Sample,
        _options: Self::Options,
    ) -> Result<SampleEvolution, Self::Error> {
        let fragment = scraper::Html::parse_fragment(input.content());

        Ok(SampleEvolution::Transform(
            input.evolve(
                Trace::new(HashMap::new()),
                fragment
                    .root_element()
                    .first_element_child()
                    .ok_or_else(|| FragmentError::NoElement)?
                    .text()
                    .collect::<Vec<_>>()
                    .join(""),
            ),
        ))
    }
}
