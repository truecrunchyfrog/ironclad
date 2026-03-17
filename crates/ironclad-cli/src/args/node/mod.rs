pub(crate) mod add;
pub(crate) mod edit;
pub(crate) mod list;
pub(crate) mod remove;
pub(crate) mod reuse;
pub(crate) mod show;

use clap::Subcommand;

use crate::args::node::{
    add::AddNodeArgs, edit::EditNodeArgs, list::ListNodeArgs, remove::RemoveNodeArgs,
    reuse::ReuseNodeArgs, show::ShowNodeArgs,
};

#[derive(Subcommand)]
pub(crate) enum NodeCommand {
    #[command(name = "use")]
    Reuse(ReuseNodeArgs),
    Add(AddNodeArgs),
    Edit(EditNodeArgs),
    Remove(RemoveNodeArgs),
    List(ListNodeArgs),
    #[command(alias = "s")]
    Show(ShowNodeArgs),
}
