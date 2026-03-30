use ironclad_core::{pipeline::Stage, registry};

use crate::{
    args::pipeline::push::PushPipelineArgs,
    helper::{resolve_explicit_or_reused_cell, resolve_ledger},
};

pub(super) fn dispatch(args: PushPipelineArgs) -> anyhow::Result<()> {
    let ledger = resolve_ledger()?;
    let mut cell = resolve_explicit_or_reused_cell(&ledger, Some(args.cell_id))?;

    registry::resolve_op(&args.operation_id)?;

    let options = match args.options {
        Some(serialized) => serde_json::from_str::<serde_json::Value>(&serialized)?,
        None => serde_json::Value::Null,
    };

    let stage = Stage::new(args.operation_id, options);

    cell.pipeline_mut().add(args.index, stage)?;

    ledger.save_cell(&cell)?;

    Ok(())
}
