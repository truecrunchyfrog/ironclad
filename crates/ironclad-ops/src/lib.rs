mod fragment_error;
mod ops;
mod tag;
mod text_selector;

use ironclad_core::registry::{Registry, error::RegistryError};

pub fn register_ops(registry: &mut Registry) -> Result<(), RegistryError> {
    for (id, op) in ops::operations() {
        registry.register_op(id.to_string(), op)?;
    }
    Ok(())
}
