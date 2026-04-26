use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
};

use clap_stdin::{FileOrStdin, FileOrStdout};
use ironclad_core::{
    catalog::{Catalog, FactIndex},
    fact::{Fact, LabeledFact},
    snapshot::Snapshot,
};

pub(crate) struct CatalogSession {
    catalog: Catalog,
    index: FactIndex,
}

pub(crate) struct ResolvedFactRef {
    pub(crate) selector: String,
    pub(crate) fact_id: String,
}

pub(crate) enum SnapshotPath {
    Actual,
    Canon,
}

impl CatalogSession {
    pub(crate) fn open() -> anyhow::Result<Self> {
        let catalog = Catalog::find_for_working_dir(&std::env::current_dir()?)?;
        let index = catalog.load_fact_index()?;
        Ok(Self { catalog, index })
    }

    pub(crate) fn catalog(&self) -> &Catalog {
        &self.catalog
    }

    pub(crate) fn index(&self) -> &FactIndex {
        &self.index
    }

    pub(crate) fn index_mut(&mut self) -> &mut FactIndex {
        &mut self.index
    }

    pub(crate) fn save_index(&self) -> anyhow::Result<()> {
        Ok(self.catalog.save_fact_index(&self.index)?)
    }

    pub(crate) fn resolve_fact_ref(&self, selector: &str) -> anyhow::Result<ResolvedFactRef> {
        let fact_id = self
            .catalog
            .resolve_fact_id_in_index(&self.index, selector)?;

        Ok(ResolvedFactRef {
            selector: selector.to_string(),
            fact_id,
        })
    }

    pub(crate) fn load_fact(&self, fact_id: &str) -> anyhow::Result<Fact> {
        Ok(self
            .catalog
            .load_fact_for_path(&self.catalog.fact_file_path(fact_id))?)
    }

    pub(crate) fn labeled_facts_including(
        &self,
        labels: &[String],
    ) -> anyhow::Result<Vec<LabeledFact>> {
        Ok(self
            .catalog
            .load_labeled_facts_including(&self.index, labels)?)
    }

    pub(crate) fn labeled_facts_excluding(
        &self,
        labels: &[String],
    ) -> anyhow::Result<Vec<LabeledFact>> {
        for label in labels {
            if !self.index.contains_label(label) {
                anyhow::bail!("absent from index: {label}");
            }
        }

        Ok(self
            .catalog
            .load_labeled_facts_excluding(&self.index, labels)?)
    }
}

pub(crate) fn resolve_catalog() -> anyhow::Result<Catalog> {
    Ok(Catalog::find_for_working_dir(&std::env::current_dir()?)?)
}

pub(crate) fn read_snapshot(
    catalog: &Catalog,
    source: Option<FileOrStdin>,
    default_path: SnapshotPath,
) -> anyhow::Result<Snapshot> {
    Ok(serde_json::from_reader::<Box<dyn Read>, Snapshot>(
        match source {
            Some(file_or_stdin) => Box::new(file_or_stdin.into_reader()?),
            None => Box::new(BufReader::new(File::open(snapshot_file_path(
                catalog,
                default_path,
            ))?)),
        },
    )?)
}

pub(crate) fn write_snapshot(
    catalog: &Catalog,
    dest: Option<FileOrStdout>,
    default_path: SnapshotPath,
    snapshot: &Snapshot,
) -> anyhow::Result<()> {
    let mut dest: Box<dyn Write> = match dest {
        Some(file_or_stdout) => Box::new(file_or_stdout.into_writer()?),
        None => Box::new(BufWriter::new(File::create(snapshot_file_path(
            catalog,
            default_path,
        ))?)),
    };

    dest.write_all(serde_json::to_vec_pretty(snapshot)?.as_slice())?;
    Ok(())
}

fn snapshot_file_path(catalog: &Catalog, path: SnapshotPath) -> std::path::PathBuf {
    match path {
        SnapshotPath::Actual => catalog.snapshot_actual_file_path(),
        SnapshotPath::Canon => catalog.snapshot_canon_file_path(),
    }
}
