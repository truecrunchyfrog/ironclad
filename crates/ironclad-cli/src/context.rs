use ironclad_core::{catalog::CatalogSession, registry::Registry};

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
}
