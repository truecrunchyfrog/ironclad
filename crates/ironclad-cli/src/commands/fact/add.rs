use std::time::Duration;

use ironclad_core::fact::{Fact, id::FactId};

use crate::{args::fact::add::AddFactArgs, config::Config, helper::resolve_cluster, reuse_fact};

pub(super) fn dispatch(_config: &Config, args: AddFactArgs) -> anyhow::Result<()> {
    let fact_id: FactId = match args {
        AddFactArgs {
            fact_id: Some(fact_id),
            generate_id: false,
            ..
        } => fact_id.into(),
        AddFactArgs {
            fact_id: None,
            generate_id: true,
            ..
        } => Default::default(),
        _ => unreachable!(),
    };

    let fact = Fact::new(
        fact_id,
        args.description,
        Default::default(),
        args.cache_lifespan
            .map_or(Duration::ZERO, std::convert::Into::into),
        Default::default(),
    );

    let cluster = resolve_cluster()?;
    cluster.add_fact(&fact)?;

    println!("{}", fact.id());

    if !args.no_use {
        reuse_fact::set(&cluster, fact.id().clone(), None)?;
    }

    Ok(())
}
