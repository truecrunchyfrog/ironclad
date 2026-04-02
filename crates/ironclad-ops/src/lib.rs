mod fragment_error;
mod ops;
mod tag;
mod text_selector;

use ironclad_core::registry::{self, error::RegistryError};

pub fn register_ops() -> Result<(), RegistryError> {
    let mut registry = registry::registry().write().unwrap();
    for (id, op) in ops::operations() {
        registry.register_op(id.to_string(), op)?;
    }
    Ok(())
}
