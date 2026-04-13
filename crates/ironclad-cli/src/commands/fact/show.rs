use crate::{
    args::fact::show::ShowFactArgs,
    config::Config,
    helper::{resolve_cluster, resolve_explicit_or_reused_fact},
};

pub(super) fn dispatch(_config: &Config, args: ShowFactArgs) -> anyhow::Result<()> {
    let cluster = resolve_cluster()?;
    let fact = resolve_explicit_or_reused_fact(&cluster, args.fact_id)?;

    match args {
        ShowFactArgs { raw: true, .. } => {
            println!("{}", serde_json::to_string_pretty(&fact)?);
        }

        ShowFactArgs { path: true, .. } => {
            println!("{}", cluster.fact_path(fact.id()).to_string_lossy());
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
                fact.schema().stages().len()
            );
        }
    }

    Ok(())
}
