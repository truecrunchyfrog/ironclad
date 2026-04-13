use ironclad_core::{registry, schema::Stage};

use crate::{
    args::schema::push::PushSchemaArgs,
    config::Config,
    helper::{resolve_cluster, resolve_explicit_or_reused_fact},
};

pub(super) fn dispatch(_config: &Config, args: PushSchemaArgs) -> anyhow::Result<()> {
    let cluster = resolve_cluster()?;
    let mut fact = resolve_explicit_or_reused_fact(&cluster, Some(args.fact_id))?;

    registry::resolve_op(&args.operation_id)?;

    let options = match args.options {
        Some(serialized) => serde_json::from_str::<serde_json::Value>(&serialized)?,
        None => serde_json::Value::Null,
    };

    let stage = Stage::new(args.operation_id, options);

    fact.schema_mut().add(args.index, stage)?;

    cluster.save_fact(&fact)?;

    Ok(())
}
