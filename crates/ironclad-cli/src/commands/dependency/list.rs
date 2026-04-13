use ironclad_core::fact::id::FactId;

use crate::{args::dependency::list::ListDependencyArgs, config::Config, helper::resolve_catalog};

pub(super) fn dispatch(_config: &Config, args: ListDependencyArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;

    let all_facts = catalog.load_facts()?;

    let facts = if args.all {
        all_facts.iter().collect()
    } else {
        args.fact_id
            .into_iter()
            .map(|fact_id| FactId::from(fact_id))
            .collect::<Vec<_>>()
            .into_iter()
            .map(|fact_id| all_facts.iter().find(|fact| fact.id() == &fact_id).unwrap())
            .collect::<Vec<_>>()
    };

    for fact in &facts {
        let related_fact_ids = if args.invert {
            &all_facts
                .iter()
                .filter(|dependent_fact| dependent_fact.dependencies().contains(fact.id()))
                .map(|fact| fact.id().clone())
                .collect::<Vec<_>>()
        } else {
            fact.dependencies()
        };

        if !(related_fact_ids.is_empty() && args.skip_empty) {
            println!(
                "{} {}: {}",
                fact.id(),
                if args.invert { "needed by" } else { "needs" },
                related_fact_ids
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
    }

    Ok(())
}
