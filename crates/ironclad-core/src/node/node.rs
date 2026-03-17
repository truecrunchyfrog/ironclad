use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::{node::id::NodeId, pipeline::Pipeline};

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    #[serde(skip)]
    id: NodeId,
    description: Option<String>,
    dependencies: Vec<NodeId>,
    pipeline: Pipeline,
    cache_lifespan: Duration,
}

impl Node {
    pub fn new(
        id: NodeId,
        description: Option<String>,
        dependencies: Vec<NodeId>,
        cache_lifespan: Duration,
        pipeline: Pipeline,
    ) -> Self {
        Self {
            id,
            description,
            dependencies,
            cache_lifespan,
            pipeline,
        }
    }

    pub fn id(&self) -> &NodeId {
        &self.id
    }

    pub fn set_id(&mut self, new_id: NodeId) {
        self.id = new_id;
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn description_mut(&mut self) -> &mut Option<String> {
        &mut self.description
    }

    pub fn dependencies(&self) -> &Vec<NodeId> {
        &self.dependencies
    }

    pub fn dependencies_mut(&mut self) -> &mut Vec<NodeId> {
        &mut self.dependencies
    }

    pub fn pipeline(&self) -> &Pipeline {
        &self.pipeline
    }

    pub fn pipeline_mut(&mut self) -> &mut Pipeline {
        &mut self.pipeline
    }

    pub fn cache_lifespan(&self) -> &Duration {
        &self.cache_lifespan
    }
}
