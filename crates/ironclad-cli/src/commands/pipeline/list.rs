use crate::{
    args::pipeline::list::ListPipelineArgs,
    helper::{resolve_explicit_or_reused_node, resolve_ledger},
};

pub(super) fn dispatch(args: ListPipelineArgs) -> anyhow::Result<()> {
    let ledger = resolve_ledger()?;
    let node = resolve_explicit_or_reused_node(&ledger, args.node_id)?;
    let stages = node.pipeline().stages();

    if !args.raw {
        for stage in stages {
            println!("{}  {}", stage.operation_id(), stage.options());
        }
    } else {
        println!("{}", serde_json::to_string_pretty(stages)?);
    }

    Ok(())
}
