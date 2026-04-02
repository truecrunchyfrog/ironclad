use anyhow::anyhow;
use ironclad_core::{
    cell::{Cell, id::CellId},
    cluster::Cluster,
};

use crate::reuse_cell;

pub(crate) fn resolve_cluster() -> anyhow::Result<Cluster> {
    Ok(Cluster::find_for_working_dir(&std::env::current_dir()?)?)
}

fn explicit_or_reused_cell_id(
    cluster: &Cluster,
    specified_cell_id: Option<String>,
) -> anyhow::Result<String> {
    let reuse_cell_id = reuse_cell::get(cluster)?.map(|cell_id| cell_id.to_string());
    specified_cell_id
        .filter(|cell_id| cell_id != "-")
        .or(reuse_cell_id)
        .ok_or(anyhow!("cell ID not specified, and not reusing"))
}

pub(crate) fn resolve_explicit_or_reused_cell_id(
    cluster: &Cluster,
    specified_cell_id: Option<String>,
) -> anyhow::Result<CellId> {
    Ok(cluster.resolve_cell_id(&explicit_or_reused_cell_id(cluster, specified_cell_id)?)?)
}

pub(crate) fn resolve_explicit_or_reused_cell(
    cluster: &Cluster,
    specified_cell_id: Option<String>,
) -> anyhow::Result<Cell> {
    Ok(cluster.resolve_cell(&explicit_or_reused_cell_id(cluster, specified_cell_id)?)?)
}
