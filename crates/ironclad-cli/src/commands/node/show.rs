use crate::{
    args::node::show::ShowNodeArgs,
    helper::{resolve_explicit_or_reused_node, resolve_ledger},
};

pub(super) fn dispatch(args: ShowNodeArgs) -> anyhow::Result<()> {
    let ledger = resolve_ledger()?;
    let node = resolve_explicit_or_reused_node(&ledger, args.node_id)?;

    match args {
        ShowNodeArgs { raw: true, .. } => {
            println!("{}", serde_json::to_string_pretty(&node)?);
        }

        ShowNodeArgs { path: true, .. } => {
            println!("{}", ledger.node_path(node.id()).to_string_lossy());
        }

        _ => {
            println!(
                "{}\ndescription: {}\ndependencies: {}\nstages: {}",
                node.id(),
                node.description()
                    .clone()
                    .unwrap_or_else(|| String::from("none")),
                node.dependencies()
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" "),
                node.pipeline().stages().len()
            );
        }
    }

    Ok(())
}
