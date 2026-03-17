use std::time::SystemTime;

use crate::{args::node::reuse::ReuseNodeArgs, helper::resolve_ledger, reuse_node, ui};

pub(super) fn dispatch(args: ReuseNodeArgs) -> anyhow::Result<()> {
    let ledger = resolve_ledger()?;

    match args {
        ReuseNodeArgs {
            node_id: Some(node_id),
            duration,
            ..
        } => {
            let node_id = ledger.resolve_node_id(&node_id)?;

            println!("{node_id}");

            reuse_node::set(
                &ledger,
                node_id,
                duration.map(|d| SystemTime::now() + d.into()),
            )?;
        }

        ReuseNodeArgs { unset: true, .. } => match reuse_node::get(&ledger)? {
            Some(node_id) => {
                println!("{node_id}");
                reuse_node::remove()?;
            }

            None => {
                ui::error("no reuse node set");
            }
        },

        ReuseNodeArgs { .. } => match reuse_node::get(&ledger)? {
            Some(node_id) => println!("{node_id}"),
            None => eprintln!("no reuse node set"),
        },
    }

    Ok(())
}
