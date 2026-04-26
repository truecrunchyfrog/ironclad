use std::path::Path;

use crate::{
    catalog::{Catalog, CatalogRepository, FactIndex, SnapshotEvaluator, error::CatalogError},
    fact::{Fact, LabeledFact},
    registry::Registry,
    snapshot::Snapshot,
};

pub struct CatalogSession {
    repository: CatalogRepository,
    index: FactIndex,
}

pub struct ResolvedFactRef {
    pub selector: String,
    pub fact_id: String,
}

pub enum FactSelection {
    All,
    Include(Vec<String>),
    Exclude(Vec<String>),
}

impl CatalogSession {
    pub fn open(working_dir: &Path, catalog_dir: Option<&Path>) -> Result<Self, CatalogError> {
        let repository = CatalogRepository::open(working_dir, catalog_dir)?;
        let index = repository.load_fact_index()?;
        Ok(Self { repository, index })
    }

    #[must_use]
    pub fn catalog(&self) -> &Catalog {
        self.repository.catalog()
    }

    #[must_use]
    pub fn repository(&self) -> &CatalogRepository {
        &self.repository
    }

    #[must_use]
    pub fn index(&self) -> &FactIndex {
        &self.index
    }

    pub fn index_mut(&mut self) -> &mut FactIndex {
        &mut self.index
    }

    pub fn save_index(&self) -> Result<(), CatalogError> {
        self.repository.save_fact_index(&self.index)
    }

    pub fn resolve_fact_ref(&self, selector: &str) -> Result<ResolvedFactRef, CatalogError> {
        let fact_id = self
            .repository
            .resolve_fact_id_in_index(&self.index, selector)?;
        Ok(ResolvedFactRef {
            selector: selector.to_string(),
            fact_id,
        })
    }

    pub fn load_fact(&self, fact_id: &str) -> Result<Fact, CatalogError> {
        self.repository
            .load_fact_for_path(&self.catalog().fact_file_path(fact_id))
            .map_err(CatalogError::from)
    }

    pub fn labeled_facts_including(
        &self,
        labels: &[String],
    ) -> Result<Vec<LabeledFact>, CatalogError> {
        self.repository
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

        self.repository
            .load_labeled_facts_excluding(&self.index, labels)
    }

    pub fn labeled_facts(
        &self,
        selection: FactSelection,
    ) -> Result<Vec<LabeledFact>, CatalogError> {
        match selection {
            FactSelection::All => self.repository.load_labeled_facts(&self.index),
            FactSelection::Include(labels) => self.labeled_facts_including(&labels),
            FactSelection::Exclude(labels) => self.labeled_facts_excluding(&labels),
        }
    }

    pub fn capture_snapshot<F: FnMut(crate::catalog::SnapshotProgressEvent)>(
        &self,
        registry: &Registry,
        selection: FactSelection,
        redact_secrets: bool,
        on_progress: F,
    ) -> Result<Snapshot, CatalogError> {
        SnapshotEvaluator::new(self.catalog().clone()).capture_snapshot(
            registry,
            self.labeled_facts(selection)?,
            redact_secrets,
            on_progress,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use std::collections::HashMap;

    use crate::{
        catalog::{
            Catalog, CatalogRepository, CatalogSession, FactIndex, FactSelection,
            error::CatalogError,
        },
        operation::{OperationContext, TypedOperation},
        registry::Registry,
        sample::{Sample, Trace},
    };

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

    fn write_fact_toml(path: &Path, content: &str) {
        fs::write(path, content).expect("write fact");
    }

    #[test]
    fn open_session_from_container_override() {
        let root = temp_path("session-root");
        fs::create_dir_all(&root).expect("mkdir root");
        let catalog = Catalog::create_catalog(&root).expect("create catalog");
        let catalog_dir = catalog.dir().to_path_buf();

        let session = CatalogSession::open(&root, Some(&catalog_dir)).expect("open session");

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
        CatalogRepository::new(catalog.clone())
            .save_fact_index(&FactIndex::new())
            .expect("save empty index");

        let mut index = FactIndex::new();
        index.insert("fact".to_string(), fact_id.to_string());
        CatalogRepository::new(catalog.clone())
            .save_fact_index(&index)
            .expect("save index");

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

    struct SeedConst;

    impl TypedOperation for SeedConst {
        type Options = String;
        type Error = std::convert::Infallible;

        fn description(&self) -> &'static str {
            "Emit a constant sample."
        }

        fn eval_all(
            &self,
            _context: &OperationContext,
            _input: Vec<Sample>,
            options: Self::Options,
        ) -> Result<Vec<Sample>, Self::Error> {
            Ok(vec![Sample::new(
                Trace::new(HashMap::from([("name".to_string(), options.clone())])),
                options,
            )])
        }
    }

    #[test]
    fn capture_snapshot_uses_injected_registry_and_exports() {
        let root = temp_path("session-capture");
        fs::create_dir_all(&root).expect("mkdir root");
        let catalog = Catalog::create_catalog(&root).expect("create catalog");

        let first_id = "01FIRSTFACTID00000000000000";
        let second_id = "01SECONDFCTID00000000000000";

        write_fact_toml(
            &catalog.fact_file_path(first_id),
            r#"
exports.foo = { trace_key = "name", trace_value = "alpha" }

[[steps]]
use = "test.seed.const"
options = "alpha"
"#,
        );
        write_fact_toml(
            &catalog.fact_file_path(second_id),
            r#"
imports = ["foo"]

[[steps]]
use = "test.seed.const"
options = "$(foo)"
"#,
        );

        let mut index = FactIndex::new();
        index.insert("first".to_string(), first_id.to_string());
        index.insert("second".to_string(), second_id.to_string());
        CatalogRepository::new(catalog.clone())
            .save_fact_index(&index)
            .expect("save index");

        let session = CatalogSession::open(&root, None).expect("open session");
        let mut registry = Registry::new();
        registry
            .register_op("test.seed.const".to_string(), SeedConst.into())
            .expect("register op");

        let snapshot = session
            .capture_snapshot(&registry, FactSelection::All, false, |_| {})
            .expect("capture snapshot");

        assert_eq!(snapshot.entries()["first"].samples()[0].content(), "alpha");
        assert_eq!(snapshot.entries()["second"].samples()[0].content(), "alpha");

        fs::remove_dir_all(root).expect("cleanup");
    }

    #[test]
    fn fact_selection_exclude_rejects_unknown_labels() {
        let root = temp_path("session-selection");
        fs::create_dir_all(&root).expect("mkdir root");
        let _catalog = Catalog::create_catalog(&root).expect("create catalog");

        let session = CatalogSession::open(&root, None).expect("open session");
        let err = match session.labeled_facts(FactSelection::Exclude(vec!["missing".to_string()])) {
            Ok(_) => panic!("missing label should fail"),
            Err(err) => err,
        };

        assert!(matches!(err, CatalogError::LabelNotInIndex(label) if label == "missing"));

        fs::remove_dir_all(root).expect("cleanup");
    }
}
