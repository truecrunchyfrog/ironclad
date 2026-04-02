use crate::{
    args::cell::remove::RemoveCellArgs,
    config::Config,
    helper::{resolve_cluster, resolve_explicit_or_reused_cell_id},
};

pub(super) fn dispatch(_config: &Config, args: RemoveCellArgs) -> anyhow::Result<()> {
    let cluster = resolve_cluster()?;
    let cell_id = resolve_explicit_or_reused_cell_id(&cluster, args.cell_id)?;
    cluster.remove_cell(&cell_id)?;

    println!("{cell_id}");

    Ok(())
}
