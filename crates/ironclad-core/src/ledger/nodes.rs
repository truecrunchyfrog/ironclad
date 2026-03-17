use std::{
    fs::{self, DirEntry},
    path::Path,
};

use log::{info, warn};

use crate::{
    ledger::{error::LedgerError, ledger::Ledger},
    node::{Node, error::NodeError, id::NodeId},
};

impl Ledger {
    fn node_files(&self) -> Vec<DirEntry> {
        let nodes_dir = self.nodes_dir();
        let entries = nodes_dir
            .read_dir()
            .expect(&format!("cannot read {:#?} as directory", nodes_dir));

        entries
            .flat_map(|entry| {
                entry
                    .inspect_err(|err| warn!("strange entry: {}", err))
                    .ok()
                    .filter(|entry| {
                        entry
                            .file_type()
                            .inspect_err(|err| warn!("cannot check file type: {}", err))
                            .is_ok_and(|filetype| {
                                if filetype.is_file() {
                                    true
                                } else {
                                    warn!("non-file node entry ignored: {:#?}", entry.path());
                                    false
                                }
                            })
                    })
            })
            .collect::<Vec<_>>()
    }

    pub fn node_ids(&self) -> Vec<NodeId> {
        self.node_files()
            .iter()
            .map(|file| NodeId::for_path(&file.path()))
            .collect()
    }

    pub fn resolve_node_id(&self, id: &str) -> Result<NodeId, NodeError> {
        let node_ids = self.node_ids();

        if let Some(node_id) = node_ids.iter().find(|node_id| node_id.to_string() == id) {
            return Ok(node_id.clone());
        }

        let mut possible_ids = node_ids
            .iter()
            .filter(|node_id| node_id.to_string().starts_with(id));

        match (possible_ids.next(), possible_ids.next()) {
            (Some(node_id), None) => Ok(node_id.clone()),
            (None, _) => Err(NodeError::NoSuchNodeId(id.to_string()).into()),
            _ => Err(NodeError::AmbiguousNodeId(id.to_string()).into()),
        }
    }

    pub fn resolve_node(&self, id: &str) -> Result<Node, NodeError> {
        self.load_node_for_id(&self.resolve_node_id(id)?)
    }

    pub fn load_nodes(&self) -> Result<Vec<Node>, LedgerError> {
        Ok(self
            .node_files()
            .iter()
            .flat_map(|entry| {
                self.load_node_for_path(&entry.path())
                    .inspect_err(|err| warn!("failed to load node {:#?}: {}", entry.path(), err))
            })
            .collect::<Vec<_>>())
    }

    pub fn load_node_for_path(&self, path: &Path) -> Result<Node, NodeError> {
        if !path.try_exists()? {
            return Err(NodeError::PathNotFound(path.to_path_buf()));
        }

        let mut node: Node = serde_json::from_str(&fs::read_to_string(path)?)?;
        node.set_id(NodeId::for_path(path));

        Ok(node)
    }

    pub fn load_node_for_id(&self, id: &NodeId) -> Result<Node, NodeError> {
        self.load_node_for_path(&self.node_path(id))
    }

    pub fn save_node(&self, node: &Node) -> Result<(), NodeError> {
        let path = self.node_path(&node.id());

        if !path.try_exists()? {
            return Err(NodeError::PathNotFound(path));
        }

        write_node(&path, node)?;

        Ok(())
    }

    pub fn add_node(&self, node: &Node) -> Result<(), NodeError> {
        let path = self.node_path(&node.id());

        if path.try_exists()? {
            return Err(NodeError::PathAlreadyExists(path));
        }

        write_node(&path, node)?;

        self.save_node(node)
    }

    pub fn remove_node(&self, id: &NodeId) -> Result<(), NodeError> {
        let path = self.node_path(&id);

        if !path.try_exists()? {
            return Err(NodeError::PathNotFound(path));
        }

        fs::remove_file(path)?;

        Ok(())
    }
}

fn write_node(path: &Path, node: &Node) -> Result<(), NodeError> {
    info!("writing node at {:?}", path);
    fs::write(path, serde_json::to_vec_pretty(node)?)?;
    Ok(())
}
