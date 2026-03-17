use anyhow::anyhow;
use rnacl_core::{
    ledger::Ledger,
    node::{Node, id::NodeId},
};

use crate::reuse_node;

pub(crate) fn resolve_ledger() -> anyhow::Result<Ledger> {
    Ok(Ledger::find_for_working_dir(&std::env::current_dir()?)?)
}

fn explicit_or_reused_node_id(
    ledger: &Ledger,
    specified_node_id: Option<String>,
) -> anyhow::Result<String> {
    let reuse_node_id = reuse_node::get(ledger)?.map(|node_id| node_id.to_string());
    specified_node_id
        .filter(|node_id| node_id != "-")
        .or(reuse_node_id)
        .ok_or(anyhow!("node ID not specified, and not reusing"))
}

pub(crate) fn resolve_explicit_or_reused_node_id(
    ledger: &Ledger,
    specified_node_id: Option<String>,
) -> anyhow::Result<NodeId> {
    Ok(ledger.resolve_node_id(&explicit_or_reused_node_id(ledger, specified_node_id)?)?)
}

pub(crate) fn resolve_explicit_or_reused_node(
    ledger: &Ledger,
    specified_node_id: Option<String>,
) -> anyhow::Result<Node> {
    Ok(ledger.resolve_node(&explicit_or_reused_node_id(ledger, specified_node_id)?)?)
}
