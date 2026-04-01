mod eval;
mod list;
mod pop;
mod push;

use crate::{args::pipeline::PipelineCommand, config::Config};

pub(super) fn dispatch(config: &Config, command: PipelineCommand) -> anyhow::Result<()> {
    match command {
        PipelineCommand::Push(args) => push::dispatch(config, args),
        PipelineCommand::Pop(args) => pop::dispatch(config, args),
        PipelineCommand::Eval(args) => eval::dispatch(config, args),
        PipelineCommand::List(args) => list::dispatch(config, args),
    }
}
