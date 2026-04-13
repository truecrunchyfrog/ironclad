use std::time::Duration;

use ironclad_core::fact::{Fact, id::FactId};

use crate::{args::fact::add::AddFactArgs, config::Config, helper::resolve_catalog, reuse_fact};

pub(super) fn dispatch(_config: &Config, args: AddFactArgs) -> anyhow::Result<()> {
    let fact_id: FactId = args.fact_id.into();

    let fact = Fact::new(
        fact_id,
        args.description,
        Default::default(),
        args.cache_lifespan
            .map_or(Duration::ZERO, std::convert::Into::into),
        Default::default(),
    );

    let catalog = resolve_catalog()?;
    catalog.add_fact(&fact)?;

    println!("{}", fact.id());

    if !args.no_use {
        reuse_fact::set(&catalog, fact.id().clone(), None)?;
    }

    Ok(())
}
