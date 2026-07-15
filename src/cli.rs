use crate::commands::Commands;
use clap::Parser;

#[derive(Parser)]
#[command(name = "sysgraph")]
#[command(version = "1.0")]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}
