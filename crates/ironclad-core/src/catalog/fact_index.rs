use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::catalog::{Catalog, error::CatalogError};

#[derive(Serialize, Deserialize)]
pub struct FactIndex(BTreeMap<String, String>);

impl Default for FactIndex {
    fn default() -> Self {
        Self::new()
    }
}

impl FactIndex {
    #[must_use]
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    #[must_use]
    pub fn into_entries(self) -> BTreeMap<String, String> {
        self.0
    }

    #[must_use]
    pub fn entries(&self) -> &BTreeMap<String, String> {
        &self.0
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.0.iter()
    }

    pub fn id_for_label(&self, label: &str) -> Option<&str> {
        self.0.get(label).map(String::as_str)
    }

    pub fn label_for_id(&self, fact_id: &str) -> Option<&str> {
        self.0.iter().find_map(|(label, fact_id2)| {
            if fact_id2 == fact_id {
                Some(label.as_str())
            } else {
                None
            }
        })
    }

    pub fn insert(&mut self, label: String, fact_id: String) -> Option<String> {
        self.0.insert(label, fact_id)
    }

    pub fn remove_label(&mut self, label: &str) -> Option<String> {
        self.0.remove(label)
    }

    pub fn remove_fact_id(&mut self, fact_id: &str) -> Option<String> {
        let label = self.label_for_id(fact_id)?.to_string();
        self.0.remove(&label)?;
        Some(label)
    }

    pub fn contains_label(&self, label: &str) -> bool {
        self.0.contains_key(label)
    }
}

impl Catalog {
    pub fn load_fact_index(&self) -> Result<FactIndex, CatalogError> {
        Ok(toml::from_slice(
            std::fs::read(self.fact_index_file_path())?.as_slice(),
        )?)
    }

    pub fn save_fact_index(&self, fact_index: &FactIndex) -> Result<(), CatalogError> {
        Ok(std::fs::write(
            self.fact_index_file_path(),
            toml::to_string_pretty(fact_index)?,
        )?)
    }
}
