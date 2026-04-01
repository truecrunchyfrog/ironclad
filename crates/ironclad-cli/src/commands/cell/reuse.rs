use std::time::SystemTime;

use crate::{
    args::cell::reuse::ReuseCellArgs, config::Config, helper::resolve_ledger, reuse_cell, ui,
};

pub(super) fn dispatch(_config: &Config, args: ReuseCellArgs) -> anyhow::Result<()> {
    let ledger = resolve_ledger()?;

    match args {
        ReuseCellArgs {
            cell_id: Some(cell_id),
            duration,
            ..
        } => {
            let cell_id = ledger.resolve_cell_id(&cell_id)?;

            println!("{cell_id}");

            reuse_cell::set(
                &ledger,
                cell_id,
                duration.map(|d| SystemTime::now() + d.into()),
            )?;
        }

        ReuseCellArgs { unset: true, .. } => match reuse_cell::get(&ledger)? {
            Some(cell_id) => {
                println!("{cell_id}");
                reuse_cell::remove()?;
            }

            None => {
                ui::error("no reuse cell set");
            }
        },

        ReuseCellArgs { .. } => match reuse_cell::get(&ledger)? {
            Some(cell_id) => println!("{cell_id}"),
            None => eprintln!("no reuse cell set"),
        },
    }

    Ok(())
}
