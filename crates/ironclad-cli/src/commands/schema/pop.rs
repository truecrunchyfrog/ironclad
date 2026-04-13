use anyhow::anyhow;

use crate::{
    args::schema::pop::PopSchemaArgs,
    config::Config,
    helper::{resolve_cluster, resolve_explicit_or_reused_fact},
    ui,
};

pub(super) fn dispatch(_config: &Config, args: PopSchemaArgs) -> anyhow::Result<()> {
    let cluster = resolve_cluster()?;
    let mut fact = resolve_explicit_or_reused_fact(&cluster, args.fact_id)?;

    if fact.schema().stages().is_empty() {
        return Err(anyhow!("empty schema"));
    }

    let removed_stage = fact.schema_mut().remove(args.index)?;

    cluster.save_fact(&fact)?;

    ui::info(format!(
        "removed operation '{}' with options '{}'",
        removed_stage.operation_id(),
        removed_stage.options()
    ));

    Ok(())
}
