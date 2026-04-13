use anyhow::anyhow;

use crate::{
    args::schema::pop::PopSchemaArgs,
    config::Config,
    helper::{resolve_catalog, resolve_explicit_or_reused_fact},
    ui,
};

pub(super) fn dispatch(_config: &Config, args: PopSchemaArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;
    let mut fact = resolve_explicit_or_reused_fact(&catalog, args.fact_id)?;

    if fact.schema().stages().is_empty() {
        return Err(anyhow!("empty schema"));
    }

    let removed_stage = fact.schema_mut().remove(args.index)?;

    catalog.save_fact(&fact)?;

    ui::info(format!(
        "removed operation '{}' with options '{}'",
        removed_stage.operation_id(),
        removed_stage.options()
    ));

    Ok(())
}
