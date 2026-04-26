use crate::{args::operation::list::ListOperationArgs, context::Context};

pub(super) fn dispatch(context: &Context, _args: ListOperationArgs) -> anyhow::Result<()> {
    let registry = context.registry();
    let mut ops = registry.ops().iter().collect::<Vec<_>>();
    ops.sort_by(|a, b| a.0.cmp(b.0));

    for (id, _) in ops {
        println!("{id}");
    }

    Ok(())
}
