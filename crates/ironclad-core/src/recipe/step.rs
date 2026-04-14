use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    catalog::Catalog,
    recipe::RecipeError,
    registry::{self},
    sample::Sample,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Step {
    operation_id: String,
    options: Value,
}

impl Step {
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
        catalog: &Catalog,
        input: Vec<Vec<Sample>>,
    ) -> Result<Vec<Vec<Sample>>, RecipeError> {
        let operation = registry::resolve_op(&self.operation_id)?;
        Ok(operation.eval(catalog, input, self.options.clone())?)
    }
}
