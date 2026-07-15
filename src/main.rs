use clap::Parser;
use sysgraph::{cli::Cli, commands::Commands};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Ps => todo!(),
        Commands::Inspect { pid } => todo!(),
        Commands::Tree { root } => todo!(),
        Commands::Dot { root } => todo!(),
    }
}
