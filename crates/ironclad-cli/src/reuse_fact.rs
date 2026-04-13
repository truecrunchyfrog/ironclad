use std::{
    env::home_dir,
    fs,
    path::PathBuf,
    time::{Duration, SystemTime},
};

use anyhow::anyhow;
use ironclad_core::{fact::id::FactId, catalog::Catalog};
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ReuseFact {
    catalog_dir: PathBuf,
    fact_id: FactId,
    expire: SystemTime,
}

fn reuse_fact_path() -> anyhow::Result<PathBuf> {
    home_dir()
        .map(|path| path.join(".ironclad_reuse_fact"))
        .ok_or_else(|| anyhow!("user has no home directory. cannot get file path for fact reuse."))
}

pub(crate) fn get(catalog: &Catalog) -> anyhow::Result<Option<FactId>> {
    let read_file = fs::read_to_string(reuse_fact_path()?);

    let default_fact = match read_file {
        Ok(content) => serde_json::from_str::<ReuseFact>(&content),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(err) => return Err(err.into()),
    }?;

    if default_fact.expire < SystemTime::now() {
        info!("reuse fact expired, removing it");
        remove()?;
        return Ok(None);
    }

    if default_fact.catalog_dir != catalog.dir() {
        return Ok(None);
    }

    Ok(Some(default_fact.fact_id))
}

pub(crate) fn set(
    catalog: &Catalog,
    fact_id: FactId,
    expire: Option<SystemTime>,
) -> anyhow::Result<()> {
    let default_fact_path = reuse_fact_path()?;
    let default_fact = ReuseFact {
        catalog_dir: catalog.dir().to_path_buf(),
        fact_id,
        expire: expire.unwrap_or_else(|| SystemTime::now() + Duration::from_mins(30)),
    };

    let contents = serde_json::to_vec(&default_fact)?;
    info!(
        "reusing fact at {:#?} with {}",
        default_fact_path, default_fact.fact_id
    );
    fs::write(default_fact_path, &contents)?;

    Ok(())
}

pub(crate) fn remove() -> anyhow::Result<()> {
    let default_fact_path = reuse_fact_path()?;

    info!("removing reuse fact at {default_fact_path:#?}");
    fs::remove_file(default_fact_path)?;

    Ok(())
}
