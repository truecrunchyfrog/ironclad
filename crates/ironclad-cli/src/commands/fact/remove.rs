use crate::{
    args::fact::remove::RemoveFactArgs,
    config::Config,
    helper::{resolve_catalog, resolve_explicit_or_reused_fact_id},
};

pub(super) fn dispatch(_config: &Config, args: RemoveFactArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;
    let fact_id = resolve_explicit_or_reused_fact_id(&catalog, args.fact_id)?;
    catalog.remove_fact(&fact_id)?;

    println!("{fact_id}");

    Ok(())
}
