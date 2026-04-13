use crate::{
    args::recipe::list::ListRecipeArgs,
    config::Config,
    helper::{resolve_catalog, resolve_explicit_or_reused_fact},
};

pub(super) fn dispatch(_config: &Config, args: ListRecipeArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;
    let fact = resolve_explicit_or_reused_fact(&catalog, args.fact_id)?;
    let stages = fact.recipe().stages();

    if args.raw {
        println!("{}", serde_json::to_string_pretty(stages)?);
    } else {
        for stage in stages {
            println!("{}  {}", stage.operation_id(), stage.options());
        }
    }

    Ok(())
}
