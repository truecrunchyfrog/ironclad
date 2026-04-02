use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::{cell::id::CellId, pipeline::Pipeline};

#[derive(Serialize, Deserialize, Debug)]
pub struct Cell {
    #[serde(skip)]
    id: CellId,
    description: Option<String>,
    dependencies: Vec<CellId>,
    pipeline: Pipeline,
    cache_lifespan: Duration,
}

impl Cell {
    #[must_use] 
    pub fn new(
        id: CellId,
        description: Option<String>,
        dependencies: Vec<CellId>,
        cache_lifespan: Duration,
        pipeline: Pipeline,
    ) -> Self {
        Self {
            id,
            description,
            dependencies,
            pipeline,
            cache_lifespan,
        }
    }

    #[must_use] 
    pub fn id(&self) -> &CellId {
        &self.id
    }

    pub fn set_id(&mut self, new_id: CellId) {
        self.id = new_id;
    }

    #[must_use] 
    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn description_mut(&mut self) -> &mut Option<String> {
        &mut self.description
    }

    #[must_use] 
    pub fn dependencies(&self) -> &Vec<CellId> {
        &self.dependencies
    }

    pub fn dependencies_mut(&mut self) -> &mut Vec<CellId> {
        &mut self.dependencies
    }

    #[must_use] 
    pub fn pipeline(&self) -> &Pipeline {
        &self.pipeline
    }

    pub fn pipeline_mut(&mut self) -> &mut Pipeline {
        &mut self.pipeline
    }

    #[must_use] 
    pub fn cache_lifespan(&self) -> &Duration {
        &self.cache_lifespan
    }
}
