use log::info;

use crate::{
    args::dependency::remove::RemoveDependencyArgs,
    config::Config,
    helper::{resolve_cluster, resolve_explicit_or_reused_fact_id},
};

pub(super) fn dispatch(_config: &Config, args: RemoveDependencyArgs) -> anyhow::Result<()> {
    let cluster = resolve_cluster()?;

    let dependents = args
        .fact_id
        .into_iter()
        .map(|fact_id| resolve_explicit_or_reused_fact_id(&cluster, Some(fact_id)))
        .collect::<anyhow::Result<Vec<_>>>()?;

    let remove_deps = args
        .dependency
        .into_iter()
        .map(|fact_id| resolve_explicit_or_reused_fact_id(&cluster, Some(fact_id)))
        .collect::<anyhow::Result<Vec<_>>>()?;

    for fact_id in dependents {
        let mut fact = cluster.load_fact_for_id(&fact_id)?;
        let deps = fact.dependencies_mut();

        deps.retain(|dep| {
            if args.all || remove_deps.contains(dep) {
                info!("removing dependency from {fact_id}: {dep}");
                false
            } else {
                true
            }
        });

        cluster.save_fact(&fact)?;
    }

    Ok(())
}
