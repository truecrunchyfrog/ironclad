use clap::Args;

/// List indexed facts.
#[derive(Args)]
#[command(long_about = "List facts.\n\n\
`list` prints the labels currently present in `index.toml`. With `--verbose`, \
it prints both the label and the fact description.\n\n\
Only indexed facts appear here. Facts created with `--no-index` are not listed.")]
pub(crate) struct ListFactArgs {
    /// Show each label together with its description.
    #[arg(short, long)]
    pub(crate) verbose: bool,
}
