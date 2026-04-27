use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
};

use crate::{
    catalog::{Catalog, FactIndex, error::CatalogError},
    fact::{Fact, LabeledFact, error::FactError},
    snapshot::Snapshot,
};

pub struct CatalogRepository {
    catalog: Catalog,
}

pub enum SnapshotFile {
    Actual,
    Canon,
}

impl CatalogRepository {
    #[must_use]
    pub fn new(catalog: Catalog) -> Self {
        Self { catalog }
    }

    pub fn open(working_dir: &Path, catalog_dir: Option<&Path>) -> Result<Self, CatalogError> {
        let catalog = match catalog_dir {
            Some(path) => Catalog::open_at_path(path)?,
            None => Catalog::find_for_working_dir(working_dir)?,
        };

        Ok(Self::new(catalog))
    }

    #[must_use]
    pub fn catalog(&self) -> &Catalog {
        &self.catalog
    }

    pub fn load_fact_index(&self) -> Result<FactIndex, CatalogError> {
        Ok(toml::from_slice(
            std::fs::read(self.catalog.fact_index_file_path())?.as_slice(),
        )?)
    }

    pub fn save_fact_index(&self, fact_index: &FactIndex) -> Result<(), CatalogError> {
        Ok(std::fs::write(
            self.catalog.fact_index_file_path(),
            toml::to_string_pretty(fact_index)?,
        )?)
    }

    pub fn load_fact_for_path(&self, path: &Path) -> Result<Fact, FactError> {
        if !path.try_exists()? {
            return Err(FactError::PathNotFound(path.to_path_buf()));
        }

        Ok(toml::from_slice(std::fs::read(path)?.as_slice())?)
    }

    pub fn resolve_fact_id_in_index(
        &self,
        index: &FactIndex,
        selector: &str,
    ) -> Result<String, CatalogError> {
        if let Some(fact_id) = index.id_for_label(selector) {
            return Ok(fact_id.to_string());
        }

        let path = self.catalog.fact_file_path(selector);
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
                    fact: self.load_fact_for_path(&self.catalog.fact_file_path(fact_id))?,
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
                        &self
                            .catalog
                            .fact_file_path(&Catalog::fact_id_for_label(index, label)?),
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
                    fact: self.load_fact_for_path(&self.catalog.fact_file_path(fact_id))?,
                })
            })
            .collect()
    }

    pub fn read_snapshot(&self, snapshot_file: SnapshotFile) -> Result<Snapshot, CatalogError> {
        Ok(serde_json::from_reader(BufReader::new(File::open(
            self.snapshot_file_path(snapshot_file),
        )?))?)
    }

    pub fn write_snapshot(
        &self,
        snapshot_file: SnapshotFile,
        snapshot: &Snapshot,
    ) -> Result<(), CatalogError> {
        Ok(serde_json::to_writer_pretty(
            BufWriter::new(File::create(self.snapshot_file_path(snapshot_file))?),
            snapshot,
        )?)
    }

    fn snapshot_file_path(&self, snapshot_file: SnapshotFile) -> std::path::PathBuf {
        match snapshot_file {
            SnapshotFile::Actual => self.catalog.snapshot_actual_file_path(),
            SnapshotFile::Canon => self.catalog.snapshot_canon_file_path(),
        }
    }
}
