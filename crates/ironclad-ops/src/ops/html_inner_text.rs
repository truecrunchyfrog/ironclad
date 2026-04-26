use std::collections::HashMap;

use ironclad_core::{
    operation::{OperationContext, TypedOperation},
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

    fn eval_each(
        &self,
        _context: &OperationContext,
        input: Sample,
        _options: Self::Options,
    ) -> Result<Vec<Sample>, Self::Error> {
        let fragment = scraper::Html::parse_fragment(input.content());

        Ok(vec![
            input.evolve(
                Trace::new(HashMap::new()),
                fragment
                    .root_element()
                    .first_element_child()
                    .ok_or(FragmentError::NoElement)?
                    .text()
                    .collect::<Vec<_>>()
                    .join(""),
            ),
        ])
    }
}
