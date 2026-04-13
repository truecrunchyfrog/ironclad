use clap::Args;

/// List facts.
#[derive(Args)]
pub(crate) struct ListFactArgs {
    #[arg(short, long)]
    pub(crate) verbose: bool,
}
