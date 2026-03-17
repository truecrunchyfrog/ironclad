use std::{
    env::home_dir,
    fs,
    path::PathBuf,
    time::{Duration, SystemTime},
};

use anyhow::anyhow;
use log::info;
use ironclad_core::{ledger::Ledger, node::id::NodeId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ReuseNode {
    ledger_dir: PathBuf,
    node_id: NodeId,
    expire: SystemTime,
}

fn reuse_node_path() -> anyhow::Result<PathBuf> {
    home_dir()
        .map(|path| path.join(".ironclad_reuse_node"))
        .ok_or_else(|| anyhow!("user has no home directory. cannot get file path for node reuse."))
}

pub(crate) fn get(ledger: &Ledger) -> anyhow::Result<Option<NodeId>> {
    let read_file = fs::read_to_string(reuse_node_path()?);

    let default_node = match read_file {
        Ok(content) => serde_json::from_str::<ReuseNode>(&content),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(err) => return Err(err.into()),
    }?;

    if default_node.expire < SystemTime::now() {
        info!("reuse node expired, removing it");
        remove()?;
        return Ok(None);
    }

    if default_node.ledger_dir != ledger.dir() {
        return Ok(None);
    }

    Ok(Some(default_node.node_id))
}

pub(crate) fn set(
    ledger: &Ledger,
    node_id: NodeId,
    expire: Option<SystemTime>,
) -> anyhow::Result<()> {
    let default_node_path = reuse_node_path()?;
    let default_node = ReuseNode {
        ledger_dir: ledger.dir().to_path_buf(),
        node_id: node_id,
        expire: expire.unwrap_or_else(|| SystemTime::now() + Duration::from_mins(30)),
    };

    let contents = serde_json::to_vec(&default_node)?;
    info!(
        "reusing node at {:#?} with {}",
        default_node_path, default_node.node_id
    );
    fs::write(default_node_path, &contents)?;

    Ok(())
}

pub(crate) fn remove() -> anyhow::Result<()> {
    let default_node_path = reuse_node_path()?;

    info!("removing reuse node at {:#?}", default_node_path);
    fs::remove_file(default_node_path)?;

    Ok(())
}

// TODO
// pub(crate) fn noerr_get(ledger: &Ledger) -> Option<NodeId> {
//     get(ledger)
//         .inspect_err(|e| warn!("cannot get reused node: {}", e))
//         .ok()
//         .flatten()
// }

// pub(crate) fn noerr_set(ledger: &Ledger, node_id: NodeId) {
//     let _ = set(ledger, node_id, None).inspect_err(|e| warn!("cannot reuse node: {}", e));
// }

// pub(crate) fn noerr_remove() {
//     let _ = remove().inspect_err(|e| warn!("cannot remove reused node: {}", e));
// }
