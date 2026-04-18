use serde::{Deserialize, Serialize};

use crate::recipe::Recipe;

#[derive(Serialize, Deserialize, Debug)]
pub struct Fact {
    description: Option<String>,
    steps: Recipe,
}

impl Fact {
    #[must_use]
    pub fn new(description: Option<String>, steps: Recipe) -> Self {
        Self { description, steps }
    }

    #[must_use]
    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn description_mut(&mut self) -> &mut Option<String> {
        &mut self.description
    }

    #[must_use]
    pub fn steps(&self) -> &Recipe {
        &self.steps
    }

    pub fn steps_mut(&mut self) -> &mut Recipe {
        &mut self.steps
    }
}
