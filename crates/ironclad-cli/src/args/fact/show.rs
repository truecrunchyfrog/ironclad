use clap::Args;

/// Show a fact or its path.
///
/// By default `show` prints a structured view of the fact definition so you can
/// inspect its description, imports, exports, steps, and other fields.
///
/// With `--path`, the command prints the file path instead. The selector can be a
/// label or a fact ID.
#[derive(Args)]
pub(crate) struct ShowFactArgs {
    /// Fact selector to show.
    pub(crate) selector: String,

    /// Print the fact file path instead of the fact contents.
    #[arg(short, long)]
    pub(crate) path: bool,
}
