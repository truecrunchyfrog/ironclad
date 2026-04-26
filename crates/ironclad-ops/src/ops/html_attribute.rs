use std::collections::HashMap;

use ironclad_core::{
    operation::{OperationContext, TypedOperation},
    sample::{Sample, Trace},
};
use scraper::Element;
use serde::{Deserialize, Serialize};

use crate::fragment_error::FragmentError;

pub(crate) struct HtmlAttribute;

#[derive(Deserialize, Serialize, Clone, Default)]
#[serde(deny_unknown_fields)]
pub(crate) struct Options {
    attribute: String,
}

impl TypedOperation for HtmlAttribute {
    type Options = Options;
    type Error = FragmentError;

    fn description(&self) -> &'static str {
        "Select the value of an HTML element's attribute."
    }

    fn eval_each(
        &self,
        _context: &OperationContext,
        input: Sample,
        options: Self::Options,
    ) -> Result<Vec<Sample>, Self::Error> {
        let fragment = scraper::Html::parse_fragment(input.content());

        Ok(vec![
            input.evolve(
                Trace::new(HashMap::new()),
                fragment
                    .root_element()
                    .first_element_child()
                    .ok_or(FragmentError::NoElement)?
                    .attr(&options.attribute)
                    .unwrap_or_default()
                    .to_string(),
            ),
        ])
    }
}
