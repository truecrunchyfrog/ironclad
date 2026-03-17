mod eval;
mod list;
mod pop;
mod push;

use crate::args::pipeline::PipelineCommand;

pub(super) fn dispatch(command: PipelineCommand) -> anyhow::Result<()> {
    match command {
        PipelineCommand::Push(args) => push::dispatch(args),
        PipelineCommand::Pop(args) => pop::dispatch(args),
        PipelineCommand::Eval(args) => eval::dispatch(args),
        PipelineCommand::List(args) => list::dispatch(args),
    }
}
