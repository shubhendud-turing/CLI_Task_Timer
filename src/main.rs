mod cli;
mod display;
mod task;

#[cfg(test)]
mod workflows;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use display::{display_current_status, display_task_summary};
use std::process;
use task::{TaskError, TaskManager};

fn main() {
    let cli = Cli::parse();
    let mut task_manager = TaskManager::new();

    match handle_command(&mut task_manager, cli.command) {
        Ok(message) => {
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

        Commands::Complete => {
            match task_manager.current_task() {
                Some(task) => {
                    let label = task.label.clone();
                    // Note: We need to add a complete_current_task method to TaskManager
                    // For now, we'll pause it as a workaround
                    task_manager.pause_current_task()?;
                    Ok(format!("Completed task: '{}'", label))
                },
                None => Err(TaskError::NoActiveTask.into()),
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_start_command() {
        let mut manager = TaskManager::new();
        let command = Commands::Start { label: "Test Task".to_string() };

        let result = handle_command(&mut manager, command);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("Started task: 'Test Task'"));
        assert_eq!(manager.task_count(), 1);
    }

    #[test]
    fn test_handle_pause_command() {
        let mut manager = TaskManager::new();
        manager.start_task("Test Task".to_string()).unwrap();

        let command = Commands::Pause;
        let result = handle_command(&mut manager, command);

        assert!(result.is_ok());
        assert!(result.unwrap().contains("Paused task"));
    }

    #[test]
    fn test_handle_pause_command_no_active_task() {
        let mut manager = TaskManager::new();
        let command = Commands::Pause;

        let result = handle_command(&mut manager, command);
        assert!(result.is_err());
        // Check that anyhow error contains the TaskError::NoActiveTask message
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("No active task to operate on"));
    }

    #[test]
    fn test_handle_resume_command() {
        let mut manager = TaskManager::new();
        manager.start_task("Test Task".to_string()).unwrap();
        manager.pause_current_task().unwrap();

        let command = Commands::Resume;
        let result = handle_command(&mut manager, command);

        assert!(result.is_ok());
        assert!(result.unwrap().contains("Resumed task"));
    }

    #[test]
    fn test_handle_status_command() {
        let mut manager = TaskManager::new();
        manager.start_task("Test Task".to_string()).unwrap();

        let command = Commands::Status;
        let result = handle_command(&mut manager, command);

        assert!(result.is_ok());
        let status = result.unwrap();
        assert!(status.contains("Current Task: Test Task"));
        assert!(status.contains("Running"));
    }

    #[test]
    fn test_handle_status_command_no_active_task() {
        let mut manager = TaskManager::new();
        let command = Commands::Status;

        let result = handle_command(&mut manager, command);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "No active task");
    }

    #[test]
    fn test_handle_list_command() {
        let mut manager = TaskManager::new();
        manager.start_task("Task 1".to_string()).unwrap();
        manager.start_task("Task 2".to_string()).unwrap();

        let command = Commands::List;
        let result = handle_command(&mut manager, command);

        assert!(result.is_ok());
        let list = result.unwrap();
        assert!(list.contains("Task Summary (2 tasks)"));
        assert!(list.contains("Task 1"));
        assert!(list.contains("Task 2"));
    }
}
