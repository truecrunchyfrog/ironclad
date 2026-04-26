use std::path::Path;

use crate::{
    catalog::{FactIndex, catalog::Catalog, error::CatalogError},
    fact::{Fact, LabeledFact, error::FactError},
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
            .map(ToString::to_string)
            .ok_or_else(|| CatalogError::LabelNotInIndex(label.to_string()))
    }

    pub fn label_for_fact_id(index: &FactIndex, fact_id: &str) -> Result<String, CatalogError> {
        index
            .label_for_id(fact_id)
            .map(ToString::to_string)
            .ok_or_else(|| CatalogError::IdNotInIndex(fact_id.to_string()))
    }

    pub fn load_fact_for_label(&self, label: &str) -> Result<Fact, CatalogError> {
        Ok(self.load_fact_for_path(
            &self.fact_file_path(&Self::fact_id_for_label(&self.load_fact_index()?, label)?),
        )?)
    }

    pub fn resolve_fact_id(&self, selector: &str) -> Result<String, CatalogError> {
        self.resolve_fact_id_in_index(&self.load_fact_index()?, selector)
    }

    pub fn resolve_fact_id_in_index(
        &self,
        index: &FactIndex,
        selector: &str,
    ) -> Result<String, CatalogError> {
        if let Some(fact_id) = index.id_for_label(selector) {
            return Ok(fact_id.to_string());
        }

        let path = self.fact_file_path(selector);
        if path.try_exists()? {
            return Ok(selector.to_string());
        }

        Err(CatalogError::FactNotFound(selector.to_string()))
    }

    pub fn load_labeled_facts(&self, index: &FactIndex) -> Result<Vec<LabeledFact>, CatalogError> {
        index
            .iter()
            .map(|(label, fact_id)| {
                Ok(LabeledFact {
                    label: label.clone(),
                    fact: self.load_fact_for_path(&self.fact_file_path(fact_id))?,
                })
            })
            .collect()
    }

    pub fn load_labeled_facts_including(
        &self,
        index: &FactIndex,
        labels: &[String],
    ) -> Result<Vec<LabeledFact>, CatalogError> {
        labels
            .iter()
            .map(|label| {
                Ok(LabeledFact {
                    label: label.clone(),
                    fact: self.load_fact_for_path(
                        &self.fact_file_path(&Self::fact_id_for_label(index, label)?),
                    )?,
                })
            })
            .collect()
    }

    pub fn load_labeled_facts_excluding(
        &self,
        index: &FactIndex,
        excluded_labels: &[String],
    ) -> Result<Vec<LabeledFact>, CatalogError> {
        index
            .iter()
            .filter(|(label, _)| !excluded_labels.contains(label))
            .map(|(label, fact_id)| {
                Ok(LabeledFact {
                    label: label.clone(),
                    fact: self.load_fact_for_path(&self.fact_file_path(fact_id))?,
                })
            })
            .collect()
    }
}
