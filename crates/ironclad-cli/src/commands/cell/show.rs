use crate::{
    args::cell::show::ShowCellArgs,
    config::Config,
    helper::{resolve_explicit_or_reused_cell, resolve_ledger},
};

pub(super) fn dispatch(_config: &Config, args: ShowCellArgs) -> anyhow::Result<()> {
    let ledger = resolve_ledger()?;
    let cell = resolve_explicit_or_reused_cell(&ledger, args.cell_id)?;

    match args {
        ShowCellArgs { raw: true, .. } => {
            println!("{}", serde_json::to_string_pretty(&cell)?);
        }

        ShowCellArgs { path: true, .. } => {
            println!("{}", ledger.cell_path(cell.id()).to_string_lossy());
        }

        _ => {
            println!(
                "{}\ndescription: {}\ndependencies: {}\nstages: {}",
                cell.id(),
                cell.description()
                    .clone()
                    .unwrap_or_else(|| String::from("none")),
                cell.dependencies()
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" "),
                cell.pipeline().stages().len()
            );
        }
    }

    Ok(())
}
