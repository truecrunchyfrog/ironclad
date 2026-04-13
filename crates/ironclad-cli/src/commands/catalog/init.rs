use std::env::current_dir;

use ironclad_core::catalog::Catalog;

use crate::{args::catalog::init::InitCatalogArgs, config::Config};

pub(super) fn dispatch(_config: &Config, args: InitCatalogArgs) -> anyhow::Result<()> {
    let dir = args.dir.unwrap_or(current_dir()?);
    Catalog::create_catalog(&dir)?;
    Ok(())
}
