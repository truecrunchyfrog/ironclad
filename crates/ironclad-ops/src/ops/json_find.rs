use std::collections::HashMap;

use ironclad_core::{
    catalog::Catalog,
    operation::TypedOperation,
    sample::{Sample, Trace},
};
use serde::Deserialize;
use serde_json_path::{JsonPath, LocatedNode};

pub(crate) struct JsonFind;

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub(crate) struct Options {
    path: JsonPath,
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

impl TypedOperation for JsonFind {
    type Options = Options;
    type Error = Error;

    fn description(&self) -> &'static str {
        "Find values in a JSON object or array. https://docs.rs/serde_json_path/latest/serde_json_path/struct.JsonPath.html"
    }

    fn eval_each(
        &self,
        _catalog: &Catalog,
        input: Sample,
        options: Self::Options,
    ) -> Result<Vec<Sample>, Self::Error> {
        let json = serde_json::from_str::<serde_json::Value>(input.content())?;
        let values = options.path.query_located(&json);

        fn located_node_to_sample(input: &Sample, located_node: LocatedNode) -> Sample {
            input.evolve(
                Trace::new(HashMap::from([(
                    "json_node_path".to_string(),
                    located_node.location().to_string(),
                )])),
                located_node.node().to_string(),
            )
        }

        Ok(values
            .into_iter()
            .map(|located_node| located_node_to_sample(&input, located_node))
            .collect())
    }
}
