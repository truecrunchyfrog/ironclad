pub(crate) mod eval;
pub(crate) mod list;
pub(crate) mod pop;
pub(crate) mod push;

use clap::Subcommand;

use crate::args::pipeline::{
    eval::EvalPipelineArgs, list::ListPipelineArgs, pop::PopPipelineArgs, push::PushPipelineArgs,
};

#[derive(Subcommand)]
pub(crate) enum PipelineCommand {
    Push(PushPipelineArgs),
    Pop(PopPipelineArgs),
    Eval(EvalPipelineArgs),
    List(ListPipelineArgs),
}
