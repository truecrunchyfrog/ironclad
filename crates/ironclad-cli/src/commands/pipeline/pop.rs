use anyhow::anyhow;

use crate::{
    args::pipeline::pop::PopPipelineArgs,
    config::Config,
    helper::{resolve_explicit_or_reused_cell, resolve_ledger},
    ui,
};

pub(super) fn dispatch(_config: &Config, args: PopPipelineArgs) -> anyhow::Result<()> {
    let ledger = resolve_ledger()?;
    let mut cell = resolve_explicit_or_reused_cell(&ledger, args.cell_id)?;

    if cell.pipeline().stages().is_empty() {
        return Err(anyhow!("empty pipeline"));
    }

    let removed_stage = cell.pipeline_mut().remove(args.index)?;

    ledger.save_cell(&cell)?;

    ui::info(format!(
        "removed operation '{}' with options '{}'",
        removed_stage.operation_id(),
        removed_stage.options()
    ));

    Ok(())
}
