use crate::{
    args::pipeline::list::ListPipelineArgs,
    config::Config,
    helper::{resolve_explicit_or_reused_cell, resolve_cluster},
};

pub(super) fn dispatch(_config: &Config, args: ListPipelineArgs) -> anyhow::Result<()> {
    let cluster = resolve_cluster()?;
    let cell = resolve_explicit_or_reused_cell(&cluster, args.cell_id)?;
    let stages = cell.pipeline().stages();

    if args.raw {
        println!("{}", serde_json::to_string_pretty(stages)?);
    } else {
        for stage in stages {
            println!("{}  {}", stage.operation_id(), stage.options());
        }
    }

    Ok(())
}
