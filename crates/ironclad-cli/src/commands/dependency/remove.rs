use ironclad_core::fact::id::FactId;
use log::info;

use crate::{
    args::dependency::remove::RemoveDependencyArgs, config::Config, helper::resolve_catalog,
};

pub(super) fn dispatch(_config: &Config, args: RemoveDependencyArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;

    let dependents = args
        .fact_id
        .into_iter()
        .map(|fact_id| FactId::from(fact_id))
        .collect::<Vec<_>>();

    let remove_deps = args
        .dependency
        .into_iter()
        .map(|fact_id| FactId::from(fact_id))
        .collect::<Vec<_>>();

    for fact_id in dependents {
        let mut fact = catalog.load_fact_for_id(&fact_id)?;
        let deps = fact.dependencies_mut();

        deps.retain(|dep| {
            if args.all || remove_deps.contains(dep) {
                info!("removing dependency from {fact_id}: {dep}");
                false
            } else {
                true
            }
        });

        catalog.save_fact(&fact)?;
    }

    Ok(())
}
