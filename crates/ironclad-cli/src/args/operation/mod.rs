pub(crate) mod eval;
pub(crate) mod list;

use clap::Subcommand;

use crate::args::operation::{eval::EvalOperationArgs, list::ListOperationArgs};

#[derive(Subcommand)]
pub(crate) enum OperationCommand {
    Eval(EvalOperationArgs),
    List(ListOperationArgs),
}
