use crate::{
    args::fact::show::ShowFactArgs,
    config::Config,
    helper::{resolve_catalog, resolve_fact},
};

pub(crate) fn dispatch(_config: &Config, args: ShowFactArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;
    let resolved = resolve_fact(&catalog, &args.selector)?;
    let path = catalog.fact_file_path(&resolved.fact_id);

    if args.path {
        println!("{}", path.to_string_lossy());
    } else {
        println!(
            "{}",
            resolved.fact.description().clone().unwrap_or_default()
        );
    }

    Ok(())
}
