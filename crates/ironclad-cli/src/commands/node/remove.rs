use crate::{
    args::node::remove::RemoveNodeArgs,
    helper::{resolve_explicit_or_reused_node_id, resolve_ledger},
};

pub(super) fn dispatch(args: RemoveNodeArgs) -> anyhow::Result<()> {
    let ledger = resolve_ledger()?;
    let node_id = resolve_explicit_or_reused_node_id(&ledger, args.node_id)?;
    ledger.remove_node(&node_id)?;

    println!("{}", node_id);

    Ok(())
}
