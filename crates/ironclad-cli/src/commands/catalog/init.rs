use std::env::current_dir;

use ironclad_core::catalog::Catalog;

use crate::{args::catalog::init::InitCatalogArgs, context::Context};

pub(crate) fn dispatch(context: &Context, args: InitCatalogArgs) -> anyhow::Result<()> {
    let dir = args
        .dir
        .or_else(|| context.config().catalog_dir.clone())
        .unwrap_or(current_dir()?);
    Catalog::create_catalog(&dir)?;
    Ok(())
}
