use crate::{
    args::fact::show::ShowFactArgs,
    config::Config,
    helper::{resolve_catalog, resolve_explicit_or_reused_fact},
};

pub(super) fn dispatch(_config: &Config, args: ShowFactArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;
    let fact = resolve_explicit_or_reused_fact(&catalog, args.fact_id)?;

    match args {
        ShowFactArgs { raw: true, .. } => {
            println!("{}", serde_json::to_string_pretty(&fact)?);
        }

        ShowFactArgs { path: true, .. } => {
            println!("{}", catalog.fact_path(fact.id()).to_string_lossy());
        }

        _ => {
            println!(
                "{}\ndescription: {}\ndependencies: {}\nstages: {}",
                fact.id(),
                fact.description()
                    .clone()
                    .unwrap_or_else(|| String::from("none")),
                fact.dependencies()
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" "),
                fact.recipe().stages().len()
            );
        }
    }

    Ok(())
}
