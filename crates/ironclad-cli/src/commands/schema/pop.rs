use anyhow::anyhow;

use crate::{
    args::schema::pop::PopSchemaArgs,
    config::Config,
    helper::{resolve_cluster, resolve_explicit_or_reused_cell},
    ui,
};

pub(super) fn dispatch(_config: &Config, args: PopSchemaArgs) -> anyhow::Result<()> {
    let cluster = resolve_cluster()?;
    let mut cell = resolve_explicit_or_reused_cell(&cluster, args.cell_id)?;

    if cell.schema().stages().is_empty() {
        return Err(anyhow!("empty schema"));
    }

    let removed_stage = cell.schema_mut().remove(args.index)?;

    cluster.save_cell(&cell)?;

    ui::info(format!(
        "removed operation '{}' with options '{}'",
        removed_stage.operation_id(),
        removed_stage.options()
    ));

    Ok(())
}
