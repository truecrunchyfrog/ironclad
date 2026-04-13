use crate::{
    args::fact::remove::RemoveFactArgs,
    config::Config,
    helper::{resolve_cluster, resolve_explicit_or_reused_fact_id},
};

pub(super) fn dispatch(_config: &Config, args: RemoveFactArgs) -> anyhow::Result<()> {
    let cluster = resolve_cluster()?;
    let fact_id = resolve_explicit_or_reused_fact_id(&cluster, args.fact_id)?;
    cluster.remove_fact(&fact_id)?;

    println!("{fact_id}");

    Ok(())
}
