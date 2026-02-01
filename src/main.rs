mod app;
mod args;
mod storage;
mod task;

use crate::args::{Cli, Commands};
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parser();

    match cli.commands {
        Commands::done(),
        Commands::save(),
        Commands::delete(),
        Commands::add(),
        Commands::list(),
    }

    Ok(())
}
