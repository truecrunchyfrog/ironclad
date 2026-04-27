pub(crate) mod eval;
pub(crate) mod list;
pub(crate) mod show;

use clap::Subcommand;

use crate::args::operation::{
    eval::EvalOperationArgs, list::ListOperationArgs, show::ShowOperationArgs,
};

/// Experiment with recipe operations.
#[derive(Subcommand)]
pub(crate) enum OperationCommand {
    Eval(EvalOperationArgs),
    #[command(alias = "ls")]
    List(ListOperationArgs),
    #[command(alias = "sh")]
    Show(ShowOperationArgs),
}
