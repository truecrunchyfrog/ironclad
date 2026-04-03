use crate::{
    args::dependency::list::ListDependencyArgs,
    config::Config,
    helper::{resolve_cluster, resolve_explicit_or_reused_cell_id},
};

pub(super) fn dispatch(_config: &Config, args: ListDependencyArgs) -> anyhow::Result<()> {
    let cluster = resolve_cluster()?;

    let all_cells = cluster.load_cells()?;

    let cells = if args.all {
        all_cells.iter().collect()
    } else {
        args.cell_id
            .into_iter()
            .map(|cell_id| resolve_explicit_or_reused_cell_id(&cluster, Some(cell_id)))
            .collect::<anyhow::Result<Vec<_>>>()?
            .into_iter()
            .map(|cell_id| all_cells.iter().find(|cell| cell.id() == &cell_id).unwrap())
            .collect::<Vec<_>>()
    };

    for cell in &cells {
        let related_cell_ids = if args.invert {
            &all_cells
                .iter()
                .filter(|dependent_cell| dependent_cell.dependencies().contains(cell.id()))
                .map(|cell| cell.id().clone())
                .collect::<Vec<_>>()
        } else {
            cell.dependencies()
        };

        if !(related_cell_ids.is_empty() && args.skip_empty) {
            println!(
                "{} {}: {}",
                cell.id(),
                if args.invert { "needed by" } else { "needs" },
                related_cell_ids
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
    }

    Ok(())
}
