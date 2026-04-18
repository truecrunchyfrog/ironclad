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

        Ok(serde_json::from_slice(std::fs::read(path)?.as_slice())?)
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
}
