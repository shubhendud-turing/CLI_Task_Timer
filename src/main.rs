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

        Commands::Delete { index, completed } => {
            if completed {
                // Delete all completed tasks
                let count = task_manager.delete_completed_tasks()?;
                if count == 0 {
                    Ok("No completed tasks to delete".to_string())
                } else {
                    Ok(format!("{} completed task(s) deleted successfully", count))
                }
            } else if let Some(idx) = index {
                // Delete specific task by index
                if task_manager.task_count() == 0 {
                    return Err(TaskError::InvalidState {
                        message: "No tasks available to delete".to_string(),
                    }
                    .into());
                }

                let task_label = if idx > 0 && idx <= task_manager.task_count() {
                    task_manager.all_tasks()[idx - 1].label.clone()
                } else {
                    String::new()
                };

                task_manager.delete_task(idx)?;

                if !task_label.is_empty() {
                    Ok(format!("Task \"{}\" deleted successfully", task_label))
                } else {
                    Ok("Task deleted successfully".to_string())
                }
            } else {
                Err(TaskError::InvalidState {
                    message: "Please specify a task index or use --completed flag".to_string(),
                }
                .into())
            }
        },

        Commands::Rename { index, new_label } => {
            let old_label = task_manager.rename_task(index, new_label.clone())?;
            Ok(format!(
                "Task renamed from \"{}\" to \"{}\"",
                old_label, new_label
            ))
        },
    }
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod workflows;
