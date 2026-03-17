pub mod error;

use std::{
    collections::HashMap,
    sync::{Arc, OnceLock, RwLock},
};

use log::{info, warn};

use crate::{operation::Operation, registry::error::RegistryError};

struct Registry {
    ops: HashMap<String, Arc<dyn Operation>>,
}

static REGISTRY: OnceLock<RwLock<Registry>> = OnceLock::new();

fn registry() -> &'static RwLock<Registry> {
    REGISTRY.get_or_init(|| {
        RwLock::new(Registry {
            ops: HashMap::new(),
        })
    })
}

pub fn register_op(id: String, op: Box<dyn Operation>) -> Result<(), RegistryError> {
    let mut registry = registry().write().unwrap();

    if registry.ops.contains_key(&id) {
        warn!(
            "an operation with ID '{}' is already registered, ignoring registration",
            id
        );
        return Err(RegistryError::OperationAlreadyExists(id));
    }

    info!("registering operation '{}'", id);
    registry.ops.insert(id, op.into());

    Ok(())
}

pub fn with_all_ops<F, R>(f: F) -> R
where
    F: FnOnce(&HashMap<String, Arc<dyn Operation>>) -> R,
{
    f(&registry().read().unwrap().ops)
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
