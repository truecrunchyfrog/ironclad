mod fragment_error;
mod ops;
mod tag;
mod text_selector;

use rnacl_core::registry::{self, error::RegistryError};

pub fn register_ops() -> Result<(), RegistryError> {
    for (id, op) in ops::operations() {
        registry::register_op(id.to_string(), op)?;
    }
    Ok(())
}
