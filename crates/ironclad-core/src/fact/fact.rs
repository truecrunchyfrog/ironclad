use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::{fact::id::FactId, recipe::Recipe};

#[derive(Serialize, Deserialize, Debug)]
pub struct Fact {
    #[serde(skip)]
    id: FactId,
    description: Option<String>,
    dependencies: Vec<FactId>,
    recipe: Recipe,
    cache_lifespan: Duration,
}

impl Fact {
    #[must_use]
    pub fn new(
        id: FactId,
        description: Option<String>,
        dependencies: Vec<FactId>,
        cache_lifespan: Duration,
        recipe: Recipe,
    ) -> Self {
        Self {
            id,
            description,
            dependencies,
            recipe,
            cache_lifespan,
        }
    }

    #[must_use]
    pub fn id(&self) -> &FactId {
        &self.id
    }

    pub fn id_mut(&mut self) -> &mut FactId {
        &mut self.id
    }

    #[must_use]
    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn description_mut(&mut self) -> &mut Option<String> {
        &mut self.description
    }

    #[must_use]
    pub fn dependencies(&self) -> &Vec<FactId> {
        &self.dependencies
    }

    pub fn dependencies_mut(&mut self) -> &mut Vec<FactId> {
        &mut self.dependencies
    }

    #[must_use]
    pub fn recipe(&self) -> &Recipe {
        &self.recipe
    }

    pub fn recipe_mut(&mut self) -> &mut Recipe {
        &mut self.recipe
    }

    #[must_use]
    pub fn cache_lifespan(&self) -> &Duration {
        &self.cache_lifespan
    }

    pub fn cache_lifespan_mut(&mut self) -> &mut Duration {
        &mut self.cache_lifespan
    }
}
