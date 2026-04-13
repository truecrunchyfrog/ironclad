use ironclad_core::fact::id::FactId;
use log::info;

use crate::{args::dependency::add::AddDependencyArgs, config::Config, helper::resolve_catalog};

pub(super) fn dispatch(_config: &Config, args: AddDependencyArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;

    let dependents = args
        .fact_id
        .into_iter()
        .map(|fact_id| FactId::from(fact_id))
        .collect::<Vec<_>>();

    let new_dependencies = {
        let mut result = Vec::new();

        for fact_id in args.dependency {
            result.push(FactId::from(fact_id));
        }

        for fact_id in args.from {
            let fact = catalog.load_fact_for_id(&FactId::from(fact_id))?;
            result.extend(fact.dependencies().to_owned());
        }

        if args.mirror {
            result.extend(dependents.clone());
        }

        result
    };

    for fact_id in dependents {
        let mut fact = catalog.load_fact_for_id(&fact_id)?;
        let deps = fact.dependencies_mut();

        for new_dependency in &new_dependencies {
            if new_dependency != &fact_id && !deps.contains(new_dependency) {
                info!("adding dependency to {fact_id}: {new_dependency}");
                deps.push(new_dependency.clone());
            }
        }

        catalog.save_fact(&fact)?;
    }

    Ok(())
}
