use ironclad_core::{fact::id::FactId, recipe::Step, registry};

use crate::{args::recipe::push::PushRecipeArgs, config::Config, helper::resolve_catalog};

pub(super) fn dispatch(_config: &Config, args: PushRecipeArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;
    let mut fact = catalog.load_fact_for_id(&FactId::from(args.fact_id))?;

    registry::resolve_op(&args.operation_id)?;

    let options = match args.options {
        Some(serialized) => serde_json::from_str::<serde_json::Value>(&serialized)?,
        None => serde_json::Value::Null,
    };

    let step = Step::new(args.operation_id, options);

    fact.recipe_mut().add(args.index, step)?;

    catalog.save_fact(&fact)?;

    Ok(())
}
