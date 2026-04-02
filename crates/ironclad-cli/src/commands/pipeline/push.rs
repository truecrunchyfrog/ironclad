use ironclad_core::{pipeline::Stage, registry};

use crate::{
    args::pipeline::push::PushPipelineArgs,
    config::Config,
    helper::{resolve_explicit_or_reused_cell, resolve_cluster},
};

pub(super) fn dispatch(_config: &Config, args: PushPipelineArgs) -> anyhow::Result<()> {
    let cluster = resolve_cluster()?;
    let mut cell = resolve_explicit_or_reused_cell(&cluster, Some(args.cell_id))?;

    registry::resolve_op(&args.operation_id)?;

    let options = match args.options {
        Some(serialized) => serde_json::from_str::<serde_json::Value>(&serialized)?,
        None => serde_json::Value::Null,
    };

    let stage = Stage::new(args.operation_id, options);

    cell.pipeline_mut().add(args.index, stage)?;

    cluster.save_cell(&cell)?;

    Ok(())
}
