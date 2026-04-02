use crate::{args::cell::list::ListCellArgs, config::Config, helper::resolve_ledger};

pub(super) fn dispatch(_config: &Config, args: ListCellArgs) -> anyhow::Result<()> {
    let ledger = resolve_ledger()?;
    let cells = ledger.load_cells()?;

    let cell_id_width = cells
        .iter()
        .map(|cell| cell.id().to_string().len())
        .max()
        .unwrap_or(0);

    for cell in cells {
        if args.verbose {
            println!(
                "{:width$}  {}",
                cell.id(),
                cell.description()
                    .clone()
                    .unwrap_or_else(|| String::from("-")),
                width = cell_id_width
            );
        } else {
            println!("{}", cell.id());
        }
    }

    Ok(())
}
