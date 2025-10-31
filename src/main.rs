mod cli;
mod display;
mod task;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use display::{display_current_status, display_task_summary};
use std::process;
use task::{TaskError, TaskManager};

fn main() {
    let cli = Cli::parse();

    // Load existing state or create new TaskManager
    let mut task_manager = match TaskManager::load_or_create() {
        Ok(manager) => manager,
        Err(e) => {
            eprintln!("Warning: Could not load tasks ({}), starting fresh", e);
            TaskManager::new()
        },
    };

    match handle_command(&mut task_manager, cli.command) {
        Ok(message) => {
            // Save state after successful command
            if let Err(e) = task_manager.save() {
                eprintln!("Warning: Could not save tasks: {}", e);
            }

            if !message.is_empty() {
                println!("{}", message);
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        },
    }
}

fn handle_command(task_manager: &mut TaskManager, command: Commands) -> Result<String> {
    match command {
        Commands::Start { label } => {
            let _task_index = task_manager.start_task(label.clone())?;
            Ok(format!("Started task: '{}'", label))
        },

        Commands::Pause => {
            task_manager.pause_current_task()?;
            let current_task = task_manager.current_task();
            Ok(format!(
                "Paused task. {}",
                display_current_status(current_task)
            ))
        },

        Commands::Resume => {
            task_manager.resume_current_task()?;
            let current_task = task_manager.current_task();
            Ok(format!(
                "Resumed task. {}",
                display_current_status(current_task)
            ))
        },

        Commands::Status => {
            let current_task = task_manager.current_task();
            Ok(display_current_status(current_task))
        },

        Commands::List => Ok(display_task_summary(task_manager.all_tasks())),

        Commands::Complete => match task_manager.current_task() {
            Some(task) => {
                let label = task.label.clone();
                task_manager.complete_current_task()?;
                Ok(format!("Completed task: '{}'", label))
            },
            None => Err(TaskError::NoActiveTask.into()),
        },
    }
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod workflows;
