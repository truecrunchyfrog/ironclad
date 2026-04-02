use std::env::current_dir;

use ironclad_core::cluster::Cluster;

use crate::{args::cluster::init::InitClusterArgs, config::Config};

pub(super) fn dispatch(_config: &Config, args: InitClusterArgs) -> anyhow::Result<()> {
    let dir = args.dir.unwrap_or(current_dir()?);
    Cluster::create_cluster(&dir)?;
    Ok(())
}
