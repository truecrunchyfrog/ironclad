mod apply;
mod catalog;
mod check;
mod diff;
mod fact;
mod inspect;
mod operation;
mod resolve;

use crate::{args::Command, context::Context};

pub(super) fn dispatch(context: &Context, command: Command) -> anyhow::Result<()> {
    match command {
        Command::Init(args) => catalog::init::dispatch(context, args),
        Command::Add(args) => fact::add::dispatch(context, args),
        Command::Edit(args) => fact::edit::dispatch(context, args),
        Command::Rename(args) => fact::rename::dispatch(context, args),
        Command::Remove(args) => fact::remove::dispatch(context, args),
        Command::Show(args) => fact::show::dispatch(context, args),
        Command::List(args) => fact::list::dispatch(context, args),
        Command::Operation(cmd) => operation::dispatch(context, cmd),
        Command::Resolve(args) => resolve::dispatch(context, args),
        Command::Inspect(args) => inspect::dispatch(context, args),
        Command::Diff(args) => diff::dispatch(context, args),
        Command::Check(args) => check::dispatch(context, args),
        Command::Apply(args) => apply::dispatch(context, args),
    }
}
