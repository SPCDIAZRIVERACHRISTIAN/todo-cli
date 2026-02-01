use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[arg(short, long)]
    pub file: Option<PathBuf>,
    /// Which command to run
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Mark task as done eg. todo done 2
    Done { id: u32 },
    /// Delete task specified eg. todo delete 1
    Delete { id: u32 },
    /// Add task to list eg. todo add "task"
    Add { title: String },
    /// List all tasks in memory eg. todo list
    List,
}
