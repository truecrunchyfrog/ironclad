pub(crate) mod eval;
pub(crate) mod list;
pub(crate) mod pop;
pub(crate) mod push;

use clap::Subcommand;

use crate::args::schema::{
    eval::EvalSchemaArgs, list::ListSchemaArgs, pop::PopSchemaArgs, push::PushSchemaArgs,
};

/// View, update, and evaluate the schema of a cell.
#[derive(Subcommand)]
pub(crate) enum SchemaCommand {
    Push(PushSchemaArgs),
    Pop(PopSchemaArgs),
    Eval(EvalSchemaArgs),
    List(ListSchemaArgs),
}
