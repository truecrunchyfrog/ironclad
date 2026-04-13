use crate::{args::fact::list::ListFactArgs, config::Config, helper::resolve_cluster};

pub(super) fn dispatch(_config: &Config, args: ListFactArgs) -> anyhow::Result<()> {
    let cluster = resolve_cluster()?;
    let facts = cluster.load_facts()?;

    let fact_id_width = facts
        .iter()
        .map(|fact| fact.id().to_string().len())
        .max()
        .unwrap_or(0);

    for fact in facts {
        if args.verbose {
            println!(
                "{:width$}  {}",
                fact.id(),
                fact.description()
                    .clone()
                    .unwrap_or_else(|| String::from("-")),
                width = fact_id_width
            );
        } else {
            println!("{}", fact.id());
        }
    }

    Ok(())
}
