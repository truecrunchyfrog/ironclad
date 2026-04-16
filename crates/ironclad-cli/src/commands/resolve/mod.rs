use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::{args::resolve::ResolveArgs, config::Config, helper::resolve_catalog};

pub(super) fn dispatch(_config: &Config, args: ResolveArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;
    let snapshot = catalog.capture_snapshot(None)?;

    let mut dest: Box<dyn Write> = match args.destination {
        Some(file_or_stdout) => Box::new(file_or_stdout.into_writer()?),
        None => Box::new(BufWriter::new(File::create(
            catalog.snapshot_candidate_file_path(),
        )?)),
    };

    dest.write(serde_json::to_vec_pretty(&snapshot)?.as_slice())?;

    Ok(())
}
