use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    cluster::Cluster,
    schema::SchemaError,
    registry::{self},
    sample::Sample,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Stage {
    operation_id: String,
    options: Value,
}

impl Stage {
    #[must_use]
    pub fn new(operation_id: String, options: Value) -> Self {
        Self {
            operation_id,
            options,
        }
    }

    #[must_use]
    pub fn operation_id(&self) -> &str {
        &self.operation_id
    }

    #[must_use]
    pub fn options(&self) -> &Value {
        &self.options
    }

    pub fn eval(
        &self,
        cluster: &Cluster,
        input: Vec<Vec<Sample>>,
    ) -> Result<Vec<Vec<Sample>>, SchemaError> {
        let operation = registry::resolve_op(&self.operation_id)?;
        Ok(operation.eval(cluster, input, self.options.clone())?)
    }
}
