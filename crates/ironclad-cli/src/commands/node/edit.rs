use crate::{
    args::node::edit::EditNodeArgs,
    helper::{resolve_explicit_or_reused_node, resolve_ledger},
};

pub(super) fn dispatch(args: EditNodeArgs) -> anyhow::Result<()> {
    let ledger = resolve_ledger()?;
    let mut node = resolve_explicit_or_reused_node(&ledger, args.node_id)?;

    if let Some(description) = args.description {
        *node.description_mut() = Some(description);
    }

    if args.unset_description {
        *node.description_mut() = None;
    }

    ledger.save_node(&node)?;

    println!("{}", node.id());

    Ok(())
}
