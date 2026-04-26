use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
};

use clap_stdin::{FileOrStdin, FileOrStdout};
use ironclad_core::{catalog::Catalog, snapshot::Snapshot};

pub(crate) enum SnapshotPath {
    Actual,
    Canon,
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
