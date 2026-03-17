pub(crate) mod add;
pub(crate) mod remove;

use clap::Subcommand;

use crate::args::dependency::{add::AddDependencyArgs, remove::RemoveDependencyArgs};

#[derive(Subcommand)]
pub(crate) enum DependencyCommand {
    Add(AddDependencyArgs),
    Remove(RemoveDependencyArgs),
}
