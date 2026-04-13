use std::time::SystemTime;

use crate::{
    args::fact::reuse::ReuseFactArgs, config::Config, helper::resolve_cluster, reuse_fact, ui,
};

pub(super) fn dispatch(_config: &Config, args: ReuseFactArgs) -> anyhow::Result<()> {
    let cluster = resolve_cluster()?;

    match args {
        ReuseFactArgs {
            fact_id: Some(fact_id),
            duration,
            ..
        } => {
            let fact_id = cluster.resolve_fact_id(&fact_id)?;

            println!("{fact_id}");

            reuse_fact::set(
                &cluster,
                fact_id,
                duration.map(|d| SystemTime::now() + d.into()),
            )?;
        }

        ReuseFactArgs { unset: true, .. } => match reuse_fact::get(&cluster)? {
            Some(fact_id) => {
                println!("{fact_id}");
                reuse_fact::remove()?;
            }

            None => {
                ui::error("no reuse fact set");
            }
        },

        ReuseFactArgs { .. } => match reuse_fact::get(&cluster)? {
            Some(fact_id) => println!("{fact_id}"),
            None => eprintln!("no reuse fact set"),
        },
    }

    Ok(())
}
