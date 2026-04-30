use std::collections::HashMap;

use ironclad_core::{
    operation::{OperationContext, TypedOperation},
    sample::{Sample, Trace},
};
use scraper::Element;

use crate::fragment_error::FragmentError;

pub(crate) struct HtmlInnerHtml;

impl TypedOperation for HtmlInnerHtml {
    type Options = ();
    type Error = FragmentError;

    fn description(&self) -> &'static str {
        "Extract the inner HTML of the first HTML element in each sample.\n\n\
The sample content is parsed as an HTML fragment with the `scraper` crate:\n\
https://docs.rs/scraper/latest/scraper/\n\n\
The operation selects the first element child in the fragment and returns its \
inner HTML. If the fragment does not contain an element, the operation fails.\n\n\
Options: none."
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
                    .inner_html(),
            ),
        ])
    }
}
