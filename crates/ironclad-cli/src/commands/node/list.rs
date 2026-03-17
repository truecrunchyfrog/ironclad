use crate::{args::node::list::ListNodeArgs, helper::resolve_ledger};

pub(super) fn dispatch(_args: ListNodeArgs) -> anyhow::Result<()> {
    let ledger = resolve_ledger()?;
    let nodes = ledger.load_nodes()?;

    let node_id_width = nodes
        .iter()
        .map(|node| node.id().to_string().len())
        .max()
        .unwrap_or(0);
    for node in nodes {
        println!(
            "{:width$}  {}",
            node.id().to_string(),
            node.description()
                .clone()
                .unwrap_or_else(|| String::from("-")),
            width = node_id_width
        );
    }

    Ok(())
}
