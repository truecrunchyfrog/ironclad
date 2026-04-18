use serde::{Deserialize, Serialize};

use crate::recipe::Recipe;

#[derive(Serialize, Deserialize, Debug)]
pub struct Fact {
    description: Option<String>,
    recipe: Recipe,
}

impl Fact {
    #[must_use]
    pub fn new(description: Option<String>, recipe: Recipe) -> Self {
        Self {
            description,
            recipe,
        }
    }

    #[must_use]
    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn description_mut(&mut self) -> &mut Option<String> {
        &mut self.description
    }

    #[must_use]
    pub fn recipe(&self) -> &Recipe {
        &self.recipe
    }

    pub fn recipe_mut(&mut self) -> &mut Recipe {
        &mut self.recipe
    }
}
