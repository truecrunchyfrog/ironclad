use std::path::Path;

use crate::{
    catalog::{FactIndex, catalog::Catalog, error::CatalogError},
    fact::{Fact, error::FactError},
};

impl Catalog {
    pub fn load_fact_for_path(&self, path: &Path) -> Result<Fact, FactError> {
        if !path.try_exists()? {
            return Err(FactError::PathNotFound(path.to_path_buf()));
        }

        Ok(toml::from_slice(std::fs::read(path)?.as_slice())?)
    }

    pub fn fact_id_for_label(index: &FactIndex, label: &str) -> Result<String, CatalogError> {
        index
            .id_for_label(label)
            .ok_or_else(|| CatalogError::LabelNotInIndex(label.to_string()))
    }

    pub fn label_for_fact_id(index: &FactIndex, fact_id: &str) -> Result<String, CatalogError> {
        index
            .label_for_id(fact_id)
            .ok_or_else(|| CatalogError::IdNotInIndex(fact_id.to_string()))
    }

    pub fn load_fact_for_label(&self, label: &str) -> Result<Fact, CatalogError> {
        Ok(self.load_fact_for_path(
            &self.fact_file_path(&Self::fact_id_for_label(&self.load_fact_index()?, label)?),
        )?)
    }

    pub fn resolve_fact_id(&self, selector: &str) -> Result<String, CatalogError> {
        let index = self.load_fact_index()?;

        if let Some(fact_id) = index.id_for_label(selector) {
            return Ok(fact_id);
        }

        let path = self.fact_file_path(selector);
        if path.try_exists()? {
            return Ok(selector.to_string());
        }

        Err(CatalogError::FactNotFound(selector.to_string()))
    }
}
