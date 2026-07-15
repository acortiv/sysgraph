use clap::Parser;

use crate::{cli::Cli, commands::Commands};

mod cli;
mod commands;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Ps => todo!(),
        Commands::Inspect { pid } => todo!(),
        Commands::Tree { root } => todo!(),
        Commands::Dot { root } => todo!(),
    }
}
