use ironclad_core::{
    catalog::{Catalog, CatalogSession},
    registry::Registry,
};

use crate::config::Config;

pub(crate) struct Context {
    config: Config,
    registry: Registry,
}

impl Context {
    pub(crate) fn new(config: Config, registry: Registry) -> Self {
        Self { config, registry }
    }

    pub(crate) fn config(&self) -> &Config {
        &self.config
    }

    pub(crate) fn registry(&self) -> &Registry {
        &self.registry
    }

    pub(crate) fn catalog_session(&self) -> anyhow::Result<CatalogSession> {
        Ok(CatalogSession::open(
            &std::env::current_dir()?,
            self.config.catalog_dir.as_deref(),
        )?)
    }

    pub(crate) fn catalog(&self) -> anyhow::Result<Catalog> {
        let cwd = std::env::current_dir()?;
        Ok(match self.config.catalog_dir.as_deref() {
            Some(path) => Catalog::open_at_path(path)?,
            None => Catalog::find_for_working_dir(&cwd)?,
        })
    }

    pub(crate) fn execution_catalog(&self) -> anyhow::Result<Catalog> {
        let cwd = std::env::current_dir()?;
        Ok(match self.config.catalog_dir.as_deref() {
            Some(path) => Catalog::open_at_path(path)?,
            None => Catalog::find_for_working_dir(&cwd)
                .unwrap_or_else(|_| Catalog::for_container_dir(&cwd)),
        })
    }
}
