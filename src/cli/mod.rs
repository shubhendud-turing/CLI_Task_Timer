use clap::{Parser, Subcommand};

/// CLI Task Timer - A command-line timer for tracking time spent on tasks
#[derive(Parser)]
#[command(name = "task-timer")]
#[command(about = "A CLI tool for tracking time spent on tasks")]
#[command(version)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Start a new task with a label
    #[command(visible_alias = "s")]
    Start {
        /// Label for the task
        label: String,
    },
    /// Pause the currently running task
    #[command(visible_alias = "p")]
    Pause,
    /// Resume the currently paused task
    #[command(visible_alias = "r")]
    Resume,
    /// Show the current task status
    Status,
    /// List all tasks and their durations
    #[command(visible_alias = "l")]
    List,
    /// Complete the current task
    #[command(visible_alias = "c")]
    Complete,
    /// Delete a task by index or all completed tasks
    #[command(visible_alias = "d")]
    Delete {
        /// Index of the task to delete (1-based)
        index: Option<usize>,
        /// Delete all completed tasks
        #[arg(long)]
        completed: bool,
    },
    /// Rename a task by index
    #[command(visible_alias = "e")]
    Rename {
        /// Index of the task to rename (1-based)
        index: usize,
        /// New label for the task
        new_label: String,
    },
}

#[allow(dead_code)]
impl Commands {
    pub(crate) fn name(&self) -> &'static str {
        match self {
            Commands::Start { .. } => "start",
            Commands::Pause => "pause",
            Commands::Resume => "resume",
            Commands::Status => "status",
            Commands::List => "list",
            Commands::Complete => "complete",
            Commands::Delete { .. } => "delete",
            Commands::Rename { .. } => "rename",
        }
    }
}

#[cfg(test)]
mod cli_tests;
