pub mod error;

use std::{collections::HashMap, sync::Arc};

use log::{info, warn};

use crate::{operation::Operation, registry::error::RegistryError};

pub struct Registry {
    ops: HashMap<String, Arc<dyn Operation>>,
}

impl Registry {
    #[must_use]
    pub fn new() -> Self {
        Self {
            ops: HashMap::new(),
        }
    }

    #[must_use]
    pub fn ops(&self) -> &HashMap<String, Arc<dyn Operation>> {
        &self.ops
    }

    pub fn register_op(&mut self, id: String, op: Box<dyn Operation>) -> Result<(), RegistryError> {
        if self.ops.contains_key(&id) {
            warn!("an operation with ID '{id}' is already registered, skipping registration");
            return Err(RegistryError::OperationAlreadyExists(id));
        }

        info!("registering operation '{id}'");
        self.ops.insert(id, op.into());

        Ok(())
    }

    pub fn resolve_op(&self, id: &str) -> Result<Arc<dyn Operation>, RegistryError> {
        self.ops
            .get(id)
            .cloned()
            .ok_or_else(|| RegistryError::OperationNotFound(id.to_string()))
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}
