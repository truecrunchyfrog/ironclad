pub mod error;

use std::{
    collections::HashMap,
    sync::{Arc, OnceLock, RwLock},
};

use log::{info, warn};

use crate::{operation::Operation, registry::error::RegistryError};

pub struct Registry {
    ops: HashMap<String, Arc<dyn Operation>>,
}

impl Registry {
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
}

static REGISTRY: OnceLock<RwLock<Registry>> = OnceLock::new();

pub fn registry() -> &'static RwLock<Registry> {
    REGISTRY.get_or_init(|| {
        RwLock::new(Registry {
            ops: HashMap::new(),
        })
    })
}

pub fn resolve_op(id: &str) -> Result<Arc<dyn Operation>, RegistryError> {
    registry()
        .read()
        .unwrap()
        .ops
        .get(id)
        .cloned()
        .ok_or_else(|| RegistryError::OperationNotFound(id.to_string()))
}
