use clap::Args;
use clap_stdin::FileOrStdin;

/// Compare two snapshots.
#[derive(Args)]
#[command(long_about = "Show comparison between two snapshots.\n\n\
Without a label, `diff` prints a compact fact-level overview of changes. With a \
label, it prints sample-level changes for one fact.\n\n\
By default the command compares the resolved snapshot in `actual.json` with the \
approved snapshot in `canon.json`. Use `--proposal` or `--baseline` to override \
those inputs. `--raw` prints the diff model as JSON.")]
pub(crate) struct DiffArgs {
    /// Fact label to diff in detail.
    pub(crate) label: Option<String>,

    /// Include traces in the detailed view.
    #[arg(short, long, requires = "label")]
    pub(crate) trace: bool,

    /// Resolved snapshot to compare from.
    #[arg(short, long)]
    pub(crate) proposal: Option<FileOrStdin>,

    /// Approved snapshot to compare against.
    #[arg(short, long)]
    pub(crate) baseline: Option<FileOrStdin>,

    /// Print the diff as JSON.
    #[arg(short, long, conflicts_with = "label")]
    pub(crate) raw: bool,
}
