use std::collections::HashMap;

use ironclad_core::{
    ledger::Ledger,
    operation::{SampleEvolution, TypedOperation},
    sample::{Sample, Trace},
};
use serde::Deserialize;
use serde_json::Value;

pub(crate) struct JsonFind;

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub(crate) struct Options {
    path: Vec<PathSegment>,
}

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub(crate) enum PathSegment {
    AllValues(#[allow(dead_code)] Vec<()>),
    Index(usize),
    Key(String),
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
        "Find values in a JSON object or array."
    }

    fn eval_sample(
        &self,
        _ledger: &Ledger,
        input: Sample,
        options: Self::Options,
    ) -> Result<SampleEvolution, Self::Error> {
        let json = serde_json::from_str::<Value>(input.content())?;

        let values =
            options
                .path
                .into_iter()
                .fold(vec![(Vec::<String>::new(), json)], |values, next| {
                    values
                        .into_iter()
                        .flat_map(|(mut path, value)| match (&next, value) {
                            (PathSegment::AllValues(_), Value::Object(object)) => object
                                .into_iter()
                                .map(|(k, v)| {
                                    let mut path = path.clone();
                                    path.push(k);
                                    (path, v)
                                })
                                .collect(),
                            (PathSegment::AllValues(_), Value::Array(array)) => array
                                .into_iter()
                                .zip(0..)
                                .map(|(v, index)| {
                                    let mut path = path.clone();
                                    path.push(index.to_string());
                                    (path, v)
                                })
                                .collect(),
                            (PathSegment::Index(index), Value::Array(mut array))
                                if *index < array.len() =>
                            {
                                path.push(index.to_string());
                                vec![(path, array.swap_remove(*index))]
                            }
                            (PathSegment::Key(key), Value::Object(mut object)) => {
                                path.push(key.to_string());
                                object.remove(key).map(|v| (path, v)).into_iter().collect()
                            }
                            _ => Vec::new(),
                        })
                        .collect()
                });

        Ok(SampleEvolution::Split(
            values
                .into_iter()
                .map(|(path, value)| {
                    input.evolve(
                        Trace::new(HashMap::from([("json_path".to_string(), path.join("."))])),
                        value.to_string(),
                    )
                })
                .collect(),
        ))
    }
}
