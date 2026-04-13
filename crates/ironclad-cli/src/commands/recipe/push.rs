use ironclad_core::{registry, recipe::Stage};

use crate::{
    args::recipe::push::PushRecipeArgs,
    config::Config,
    helper::{resolve_catalog, resolve_explicit_or_reused_fact},
};

pub(super) fn dispatch(_config: &Config, args: PushRecipeArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;
    let mut fact = resolve_explicit_or_reused_fact(&catalog, Some(args.fact_id))?;

    registry::resolve_op(&args.operation_id)?;

    let options = match args.options {
        Some(serialized) => serde_json::from_str::<serde_json::Value>(&serialized)?,
        None => serde_json::Value::Null,
    };

    let stage = Stage::new(args.operation_id, options);

    fact.recipe_mut().add(args.index, stage)?;

    catalog.save_fact(&fact)?;

    Ok(())
}
