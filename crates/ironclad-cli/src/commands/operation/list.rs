use ironclad_core::registry;

use crate::{args::operation::list::ListOperationArgs, config::Config};

pub(super) fn dispatch(_config: &Config, _args: ListOperationArgs) -> anyhow::Result<()> {
    let registry = registry::registry().read().unwrap();

    let op_id_width = registry
        .ops()
        .iter()
        .map(|op| op.0.len())
        .max()
        .unwrap_or(0);
    for (id, op) in registry.ops() {
        println!("{:width$}  {}", id, op.description(), width = op_id_width);
    }

    Ok(())
}
