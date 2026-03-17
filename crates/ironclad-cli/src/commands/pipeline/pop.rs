use anyhow::anyhow;

use crate::{
    args::pipeline::pop::PopPipelineArgs,
    helper::{resolve_explicit_or_reused_node, resolve_ledger},
    ui,
};

pub(super) fn dispatch(args: PopPipelineArgs) -> anyhow::Result<()> {
    let ledger = resolve_ledger()?;
    let mut node = resolve_explicit_or_reused_node(&ledger, args.node_id)?;

    if node.pipeline().stages().is_empty() {
        return Err(anyhow!("empty pipeline"));
    }

    let removed_stage = node.pipeline_mut().remove(args.index)?;

    ledger.save_node(&node)?;

    ui::info(format!(
        "removed operation '{}' with options '{}'",
        removed_stage.operation_id(),
        removed_stage.options()
    ));

    Ok(())
}
