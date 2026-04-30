use clap::Args;

/// List indexed facts.
///
/// `list` prints the labels currently present in `index.toml`. With `--verbose`,
/// it prints both the label and the fact description.
///
/// Only indexed facts appear here. Facts created with `--no-index` are not listed.
#[derive(Args)]
pub(crate) struct ListFactArgs {
    /// Show each label together with its description.
    #[arg(short, long)]
    pub(crate) verbose: bool,
}
