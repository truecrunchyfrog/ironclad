use std::io::{Read, Write};

use clap_stdin::{FileOrStdin, FileOrStdout};
use ironclad_core::{
    catalog::{CatalogRepository, SnapshotFile},
    snapshot::Snapshot,
};

pub(crate) fn read_snapshot(
    repository: &CatalogRepository,
    source: Option<FileOrStdin>,
    default_path: SnapshotFile,
) -> anyhow::Result<Snapshot> {
    match source {
        Some(file_or_stdin) => Ok(serde_json::from_reader::<Box<dyn Read>, Snapshot>(
            Box::new(file_or_stdin.into_reader()?),
        )?),
        None => Ok(repository.read_snapshot(default_path)?),
    }
}

pub(crate) fn write_snapshot(
    repository: &CatalogRepository,
    dest: Option<FileOrStdout>,
    default_path: SnapshotFile,
    snapshot: &Snapshot,
) -> anyhow::Result<()> {
    match dest {
        Some(file_or_stdout) => {
            let mut dest: Box<dyn Write> = Box::new(file_or_stdout.into_writer()?);
            dest.write_all(serde_json::to_vec_pretty(snapshot)?.as_slice())?;
            Ok(())
        }
        None => Ok(repository.write_snapshot(default_path, snapshot)?),
    }
}
