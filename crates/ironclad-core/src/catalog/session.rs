use std::path::Path;

use crate::{
    catalog::{Catalog, FactIndex, error::CatalogError},
    fact::{Fact, LabeledFact},
};

pub struct CatalogSession {
    catalog: Catalog,
    index: FactIndex,
}

pub struct ResolvedFactRef {
    pub selector: String,
    pub fact_id: String,
}

impl CatalogSession {
    pub fn open(working_dir: &Path, catalog_dir: Option<&Path>) -> Result<Self, CatalogError> {
        let catalog = match catalog_dir {
            Some(path) => Catalog::open_at_path(path)?,
            None => Catalog::find_for_working_dir(working_dir)?,
        };
        let index = catalog.load_fact_index()?;
        Ok(Self { catalog, index })
    }

    #[must_use]
    pub fn catalog(&self) -> &Catalog {
        &self.catalog
    }

    #[must_use]
    pub fn index(&self) -> &FactIndex {
        &self.index
    }

    pub fn index_mut(&mut self) -> &mut FactIndex {
        &mut self.index
    }

    pub fn save_index(&self) -> Result<(), CatalogError> {
        self.catalog.save_fact_index(&self.index)
    }

    pub fn resolve_fact_ref(&self, selector: &str) -> Result<ResolvedFactRef, CatalogError> {
        let fact_id = self
            .catalog
            .resolve_fact_id_in_index(&self.index, selector)?;
        Ok(ResolvedFactRef {
            selector: selector.to_string(),
            fact_id,
        })
    }

    pub fn load_fact(&self, fact_id: &str) -> Result<Fact, CatalogError> {
        self.catalog
            .load_fact_for_path(&self.catalog.fact_file_path(fact_id))
            .map_err(CatalogError::from)
    }

    pub fn labeled_facts_including(
        &self,
        labels: &[String],
    ) -> Result<Vec<LabeledFact>, CatalogError> {
        self.catalog
            .load_labeled_facts_including(&self.index, labels)
    }

    pub fn labeled_facts_excluding(
        &self,
        labels: &[String],
    ) -> Result<Vec<LabeledFact>, CatalogError> {
        for label in labels {
            if !self.index.contains_label(label) {
                return Err(CatalogError::LabelNotInIndex(label.clone()));
            }
        }

        self.catalog
            .load_labeled_facts_excluding(&self.index, labels)
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use crate::catalog::{Catalog, CatalogSession, FactIndex};

    fn temp_path(name: &str) -> std::path::PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "ironclad-test-{name}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("time")
                .as_nanos()
        ));
        path
    }

    fn write_fact(path: &Path) {
        fs::write(path, "description = \"x\"\n").expect("write fact");
    }

    #[test]
    fn open_session_from_container_override() {
        let root = temp_path("session-root");
        fs::create_dir_all(&root).expect("mkdir root");
        let catalog = Catalog::create_catalog(&root).expect("create catalog");

        let session = CatalogSession::open(&root, Some(&root)).expect("open session");

        assert_eq!(session.catalog().dir(), catalog.dir());

        fs::remove_dir_all(root).expect("cleanup");
    }

    #[test]
    fn resolve_fact_ref_supports_label_and_id() {
        let root = temp_path("session-resolve");
        fs::create_dir_all(&root).expect("mkdir root");
        let catalog = Catalog::create_catalog(&root).expect("create catalog");

        let fact_id = "01TESTFACTID00000000000000";
        write_fact(&catalog.fact_file_path(fact_id));
        catalog
            .save_fact_index(&FactIndex::new())
            .expect("save empty index");

        let mut index = FactIndex::new();
        index.insert("fact".to_string(), fact_id.to_string());
        catalog.save_fact_index(&index).expect("save index");

        let session = CatalogSession::open(&root, None).expect("open session");

        assert_eq!(
            session.resolve_fact_ref("fact").expect("by label").fact_id,
            fact_id
        );
        assert_eq!(
            session.resolve_fact_ref(fact_id).expect("by id").fact_id,
            fact_id
        );

        fs::remove_dir_all(root).expect("cleanup");
    }
}
