use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// Which command to run
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Mark task as done eg. todo done 2
    done(Mark_done),
    /// Delete task specified eg. todo delete 1
    delete(Delete_task),
    /// Save tasks in to file eg. todo save
    save(Save_tasks),
    /// Add task to list eg. todo add "task"
    add(Add),
    /// List all tasks in memory eg. todo list
    list {
        /// list tasks from file instead of memory eg. todo list -f
        #[arg(short = 'f', long = "file")]
        file: bool,
    },
}
