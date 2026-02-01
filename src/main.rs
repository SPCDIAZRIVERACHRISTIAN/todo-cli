mod app;
mod args;
mod storage;
mod task;

use crate::app::App;
use crate::args::{Cli, Commands};
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let tasks = if let Some(ref path) = cli.file {
        storage::load_tasks(path)?
    } else {
        Vec::new()
    };

    let mut app = App::new(tasks);

    match cli.command {
        Commands::Add { title } => app.add(title),
        Commands::List => app.list(),
        Commands::Done { id } => app.done(id),
        Commands::Delete { id } => app.delete(id),
    }

    if let Some(ref path) = cli.file {
        storage::save_tasks(path, app.tasks())?;
    }

    Ok(())
}
