pub(crate) mod add;
pub(crate) mod edit;
pub(crate) mod list;
pub(crate) mod remove;
pub(crate) mod reuse;
pub(crate) mod show;

use clap::Subcommand;

use crate::args::cell::{
    add::AddCellArgs, edit::EditCellArgs, list::ListCellArgs, remove::RemoveCellArgs,
    reuse::ReuseCellArgs, show::ShowCellArgs,
};

#[derive(Subcommand)]
pub(crate) enum CellCommand {
    #[command(name = "use")]
    Reuse(ReuseCellArgs),
    Add(AddCellArgs),
    Edit(EditCellArgs),
    Remove(RemoveCellArgs),
    List(ListCellArgs),
    #[command(alias = "s")]
    Show(ShowCellArgs),
}
