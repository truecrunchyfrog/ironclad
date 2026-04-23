use serde::{Deserialize, Serialize};

use crate::{
    catalog::Catalog,
    recipe::RecipeError,
    registry::{self},
    sample::Sample,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Step {
    operation_id: String,
    options: serde_json::Value,
}

impl Step {
    #[must_use]
    pub fn new(operation_id: String, options: serde_json::Value) -> Self {
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
    pub fn options(&self) -> &serde_json::Value {
        &self.options
    }

    pub fn options_mut(&mut self) -> &mut serde_json::Value {
        &mut self.options
    }

    pub fn eval(&self, catalog: &Catalog, input: Vec<Sample>) -> Result<Vec<Sample>, RecipeError> {
        let operation = registry::resolve_op(&self.operation_id)?;

        operation
            .eval(catalog, input, self.options.clone())
            .map_err(|err| RecipeError::Operation {
                operation_id: self.operation_id.clone(),
                source: err,
            })
    }
}
