pub(crate) mod add;
pub(crate) mod edit;
pub(crate) mod list;
pub(crate) mod remove;
pub(crate) mod reuse;
pub(crate) mod show;

use clap::Subcommand;

use crate::args::fact::{
    add::AddFactArgs, edit::EditFactArgs, list::ListFactArgs, remove::RemoveFactArgs,
    reuse::ReuseFactArgs, show::ShowFactArgs,
};

/// Manage facts of the cluster.
#[derive(Subcommand)]
pub(crate) enum FactCommand {
    #[command(name = "use")]
    Reuse(ReuseFactArgs),
    Add(AddFactArgs),
    Edit(EditFactArgs),
    Remove(RemoveFactArgs),
    List(ListFactArgs),
    #[command(alias = "s")]
    Show(ShowFactArgs),
}
