use std::time::SystemTime;

use crate::{
    args::cell::reuse::ReuseCellArgs, config::Config, helper::resolve_cluster, reuse_cell, ui,
};

pub(super) fn dispatch(_config: &Config, args: ReuseCellArgs) -> anyhow::Result<()> {
    let cluster = resolve_cluster()?;

    match args {
        ReuseCellArgs {
            cell_id: Some(cell_id),
            duration,
            ..
        } => {
            let cell_id = cluster.resolve_cell_id(&cell_id)?;

            println!("{cell_id}");

            reuse_cell::set(
                &cluster,
                cell_id,
                duration.map(|d| SystemTime::now() + d.into()),
            )?;
        }

        ReuseCellArgs { unset: true, .. } => match reuse_cell::get(&cluster)? {
            Some(cell_id) => {
                println!("{cell_id}");
                reuse_cell::remove()?;
            }

            None => {
                ui::error("no reuse cell set");
            }
        },

        ReuseCellArgs { .. } => match reuse_cell::get(&cluster)? {
            Some(cell_id) => println!("{cell_id}"),
            None => eprintln!("no reuse cell set"),
        },
    }

    Ok(())
}
