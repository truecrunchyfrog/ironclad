use std::{
    env::home_dir,
    fs,
    path::PathBuf,
    time::{Duration, SystemTime},
};

use anyhow::anyhow;
use ironclad_core::{cell::id::CellId, ledger::Ledger};
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ReuseCell {
    ledger_dir: PathBuf,
    cell_id: CellId,
    expire: SystemTime,
}

fn reuse_cell_path() -> anyhow::Result<PathBuf> {
    home_dir()
        .map(|path| path.join(".ironclad_reuse_cell"))
        .ok_or_else(|| anyhow!("user has no home directory. cannot get file path for cell reuse."))
}

pub(crate) fn get(ledger: &Ledger) -> anyhow::Result<Option<CellId>> {
    let read_file = fs::read_to_string(reuse_cell_path()?);

    let default_cell = match read_file {
        Ok(content) => serde_json::from_str::<ReuseCell>(&content),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(err) => return Err(err.into()),
    }?;

    if default_cell.expire < SystemTime::now() {
        info!("reuse cell expired, removing it");
        remove()?;
        return Ok(None);
    }

    if default_cell.ledger_dir != ledger.dir() {
        return Ok(None);
    }

    Ok(Some(default_cell.cell_id))
}

pub(crate) fn set(
    ledger: &Ledger,
    cell_id: CellId,
    expire: Option<SystemTime>,
) -> anyhow::Result<()> {
    let default_cell_path = reuse_cell_path()?;
    let default_cell = ReuseCell {
        ledger_dir: ledger.dir().to_path_buf(),
        cell_id,
        expire: expire.unwrap_or_else(|| SystemTime::now() + Duration::from_mins(30)),
    };

    let contents = serde_json::to_vec(&default_cell)?;
    info!(
        "reusing cell at {:#?} with {}",
        default_cell_path, default_cell.cell_id
    );
    fs::write(default_cell_path, &contents)?;

    Ok(())
}

pub(crate) fn remove() -> anyhow::Result<()> {
    let default_cell_path = reuse_cell_path()?;

    info!("removing reuse cell at {:#?}", default_cell_path);
    fs::remove_file(default_cell_path)?;

    Ok(())
}
