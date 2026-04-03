use crate::{
    args::schema::list::ListSchemaArgs,
    config::Config,
    helper::{resolve_cluster, resolve_explicit_or_reused_cell},
};

pub(super) fn dispatch(_config: &Config, args: ListSchemaArgs) -> anyhow::Result<()> {
    let cluster = resolve_cluster()?;
    let cell = resolve_explicit_or_reused_cell(&cluster, args.cell_id)?;
    let stages = cell.schema().stages();

    if args.raw {
        println!("{}", serde_json::to_string_pretty(stages)?);
    } else {
        for stage in stages {
            println!("{}  {}", stage.operation_id(), stage.options());
        }
    }

    Ok(())
}
