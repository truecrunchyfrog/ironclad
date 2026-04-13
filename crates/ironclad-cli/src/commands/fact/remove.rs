use ironclad_core::fact::id::FactId;

use crate::{args::fact::remove::RemoveFactArgs, config::Config, helper::resolve_catalog};

pub(crate) fn dispatch(_config: &Config, args: RemoveFactArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;
    let fact_id = FactId::from(args.fact_id);
    catalog.remove_fact(&fact_id)?;

    println!("{fact_id}");

    Ok(())
}
