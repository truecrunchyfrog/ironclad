use std::time::Duration;

use ironclad_core::fact::id::FactId;

use crate::{args::fact::edit::EditFactArgs, config::Config, helper::resolve_catalog};

pub(super) fn dispatch(_config: &Config, args: EditFactArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;
    let mut fact = catalog.load_fact_for_id(&FactId::from(args.fact_id))?;

    if let Some(description) = args.description {
        *fact.description_mut() = Some(description);
    }

    if args.unset_description {
        *fact.description_mut() = None;
    }

    if let Some(cache_lifespan) = args.cache_lifespan {
        *fact.cache_lifespan_mut() = cache_lifespan.into();
    }

    if args.unset_cache_lifespan {
        *fact.cache_lifespan_mut() = Duration::ZERO;
    }

    if let Some(new_id) = args.id {
        let old_id = fact.id();
        let new_id: FactId = new_id.into();

        catalog.remove_fact(old_id)?;
        *fact.id_mut() = new_id.clone();
        catalog.add_fact(&fact)?;

        for mut dependent_fact in catalog.load_facts()? {
            let dependencies = dependent_fact.dependencies_mut();
            if dependencies.contains(fact.id()) {
                *dependencies = dependencies
                    .iter()
                    .filter(|&dependent_fact_id| dependent_fact_id != fact.id())
                    .cloned()
                    .collect();
                dependencies.push(new_id.clone());
                catalog.save_fact(&dependent_fact)?;
            }
        }
    } else {
        catalog.save_fact(&fact)?;
    }

    println!("{}", fact.id());

    Ok(())
}
