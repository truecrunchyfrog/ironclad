use ironclad_core::registry;

use crate::args::operation::list::ListOperationArgs;

pub(super) fn dispatch(_args: ListOperationArgs) -> anyhow::Result<()> {
    let ops = registry::with_all_ops(|ops| ops.clone());

    let op_id_width = ops.iter().map(|op| op.0.len()).max().unwrap_or(0);
    for (id, op) in ops {
        println!("{:width$}  {}", id, op.description(), width = op_id_width);
    }

    Ok(())
}
