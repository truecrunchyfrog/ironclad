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
        "Extract one attribute from the first HTML element in each sample.\n\n\
The sample content is parsed as an HTML fragment with the `scraper` crate:\n\
https://docs.rs/scraper/latest/scraper/\n\n\
The operation looks at the first element child in the fragment and returns the \
value of the requested attribute. If the attribute is missing, the result is an \
empty string. If there is no element at all, the operation fails.\n\n\
Options:\n\
- `attribute`: attribute name to read, such as `href`, `src`, or `data-id`."
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
