use std::collections::HashMap;

use ironclad_core::{
    operation::{OperationContext, TypedOperation},
    sample::{Sample, Trace},
};
use serde::{Deserialize, Serialize};
use serde_json_path::{JsonPath, LocatedNode};

pub(crate) struct JsonFind;

#[derive(Deserialize, Serialize, Clone, Default)]
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
        "Find values in JSON with JSONPath.\n\n\
The sample content is parsed as JSON with `serde_json`, then queried with \
`serde_json_path`:\n\
https://docs.rs/serde_json_path/latest/serde_json_path/\n\n\
Each match becomes one output sample. String values stay plain strings; other \
JSON values are serialized back to JSON text. A `json_node_path` trace entry is \
added for each match.\n\n\
Options:\n\
- `path`: JSONPath expression used to select values."
    }

    fn eval_each(
        &self,
        _context: &OperationContext,
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
                match located_node.node() {
                    serde_json::Value::String(s) => s.clone(),
                    otherwise => otherwise.to_string(),
                },
            )
        }

        Ok(values
            .into_iter()
            .map(|located_node| located_node_to_sample(&input, located_node))
            .collect())
    }
}
