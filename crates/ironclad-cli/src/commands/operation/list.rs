use crate::{args::operation::list::ListOperationArgs, context::Context};

pub(super) fn dispatch(context: &Context, _args: ListOperationArgs) -> anyhow::Result<()> {
    let registry = context.registry();
    let mut ops = registry.ops().iter().collect::<Vec<_>>();
    ops.sort_by(|a, b| a.0.cmp(b.0));

    let op_id_width = ops.iter().map(|(id, _)| id.len()).max().unwrap_or(0);
    for (id, op) in ops {
        println!("{:width$}  {}", id, op.description(), width = op_id_width);
    }

    Ok(())
}
