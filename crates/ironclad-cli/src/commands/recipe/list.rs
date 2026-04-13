use ironclad_core::fact::id::FactId;

use crate::{args::recipe::list::ListRecipeArgs, config::Config, helper::resolve_catalog};

pub(super) fn dispatch(_config: &Config, args: ListRecipeArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;
    let fact = catalog.load_fact_for_id(&FactId::from(args.fact_id))?;
    let steps = fact.recipe().steps();

    if args.raw {
        println!("{}", serde_json::to_string_pretty(steps)?);
    } else {
        for step in steps {
            println!("{}  {}", step.operation_id(), step.options());
        }
    }

    Ok(())
}
