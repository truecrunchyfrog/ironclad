use std::collections::HashMap;

use ironclad_core::{
    operation::{OperationContext, TypedOperation},
    sample::{Sample, Trace},
};
use serde::{Deserialize, Serialize};

pub(crate) struct HtmlFind;

#[derive(Deserialize, Serialize, Clone, Default)]
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

    fn eval_each(
        &self,
        _context: &OperationContext,
        input: Sample,
        options: Self::Options,
    ) -> Result<Vec<Sample>, Self::Error> {
        let fragment = if options.document {
            scraper::Html::parse_document
        } else {
            scraper::Html::parse_fragment
        }(input.content());
        let selector = scraper::Selector::parse(&options.selector)
            .map_err(|err| Error::Selector(err.to_string()))?;

        Ok(fragment
            .select(&selector)
            .map(|selection| {
                input.evolve(
                    Trace::new(HashMap::from([(
                        "node_id".to_string(),
                        format!("{:?}", selection.id()),
                    )])),
                    selection.html(),
                )
            })
            .collect())
    }
}
