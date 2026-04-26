use ironclad_core::{
    catalog::{CatalogRepository, CatalogSession},
    operation::OperationContext,
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

    pub(crate) fn catalog_repository(&self) -> anyhow::Result<CatalogRepository> {
        Ok(CatalogRepository::open(
            &std::env::current_dir()?,
            self.config.catalog_dir.as_deref(),
        )?)
    }

    pub(crate) fn operation_context(&self) -> anyhow::Result<OperationContext> {
        let cwd = std::env::current_dir()?;
        let repository = CatalogRepository::open(&cwd, self.config.catalog_dir.as_deref()).ok();

        Ok(match repository {
            Some(repository) => OperationContext::with_catalog(repository.catalog().clone()),
            None => OperationContext::for_working_dir(cwd),
        })
    }
}
