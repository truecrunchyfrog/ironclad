use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::catalog::{Catalog, error::CatalogError};

#[derive(Serialize, Deserialize)]
pub struct FactIndex(HashMap<String, String>);

impl FactIndex {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn into_entries(self) -> HashMap<String, String> {
        self.0
    }

    pub fn entries(&self) -> &HashMap<String, String> {
        &self.0
    }

    pub fn entries_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.0
    }

    pub fn id_for_label(&self, label: &str) -> Option<String> {
        self.0.get(label).map(ToString::to_string)
    }

    pub fn label_for_id(&self, fact_id: &str) -> Option<String> {
        self.0
            .iter()
            .find_map(|(label, fact_id2)| {
                if fact_id2 == fact_id {
                    Some(label)
                } else {
                    None
                }
            })
            .map(ToString::to_string)
    }
}

impl Catalog {
    pub fn load_fact_index(&self) -> Result<FactIndex, CatalogError> {
        Ok(serde_json::from_slice(
            std::fs::read(self.fact_index_file_path())?.as_slice(),
        )?)
    }

    pub fn save_fact_index(&self, fact_index: &FactIndex) -> Result<(), CatalogError> {
        Ok(std::fs::write(
            self.fact_index_file_path(),
            serde_json::to_vec_pretty(fact_index)?,
        )?)
    }
}
