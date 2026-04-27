use crate::catalog::{FactIndex, catalog::Catalog, error::CatalogError};

impl Catalog {
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
}
