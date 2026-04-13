use crate::{
    args::schema::list::ListSchemaArgs,
    config::Config,
    helper::{resolve_cluster, resolve_explicit_or_reused_fact},
};

pub(super) fn dispatch(_config: &Config, args: ListSchemaArgs) -> anyhow::Result<()> {
    let cluster = resolve_cluster()?;
    let fact = resolve_explicit_or_reused_fact(&cluster, args.fact_id)?;
    let stages = fact.schema().stages();

    if args.raw {
        println!("{}", serde_json::to_string_pretty(stages)?);
    } else {
        for stage in stages {
            println!("{}  {}", stage.operation_id(), stage.options());
        }
    }

    Ok(())
}
