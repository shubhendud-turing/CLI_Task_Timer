use super::*;
use crate::task::Task;
use std::time::Duration;

#[test]
fn test_format_duration_seconds_only() {
    let duration = Duration::from_secs(45);
    assert_eq!(format_duration(duration), "45s");
}

#[test]
fn test_format_duration_minutes_and_seconds() {
    let duration = Duration::from_secs(125); // 2m 5s
    assert_eq!(format_duration(duration), "2m 5s");
}

#[test]
fn test_format_duration_hours_minutes_seconds() {
    let duration = Duration::from_secs(3665); // 1h 1m 5s
    assert_eq!(format_duration(duration), "1h 1m 5s");
}

#[test]
fn test_format_duration_zero() {
    let duration = Duration::ZERO;
    assert_eq!(format_duration(duration), "0s");
}

#[test]
fn test_format_status() {
    assert!(format_status(&TaskStatus::Running).contains("Running"));
    assert!(format_status(&TaskStatus::Paused).contains("Paused"));
    assert!(format_status(&TaskStatus::Completed).contains("Completed"));
}

#[test]
fn test_display_task_with_index() {
    let task = Task::new("Test Task".to_string());
    let display = display_task(&task, Some(0));

    assert!(display.starts_with("1. Test Task"));
    assert!(display.contains("Running"));
    assert!(display.contains("Created:"));
}

#[test]
fn test_display_task_without_index() {
    let task = Task::new("Test Task".to_string());
    let display = display_task(&task, None);

    assert!(display.starts_with("Test Task"));
    assert!(!display.starts_with("1."));
    assert!(display.contains("Running"));
}

#[test]
fn test_display_current_status_with_task() {
    let task = Task::new("Active Task".to_string());
    let status = display_current_status(Some(&task));

    assert!(status.contains("Current Task: Active Task"));
    assert!(status.contains("Running"));
}

#[test]
fn test_display_current_status_no_task() {
    let status = display_current_status(None);
    assert_eq!(status, "No active task");
}

#[test]
fn test_display_task_summary_empty() {
    let tasks: Vec<Task> = vec![];
    let summary = display_task_summary(&tasks);
    assert_eq!(summary, "No tasks found");
}

#[test]
fn test_display_task_summary_with_tasks() {
    let tasks = vec![
        Task::new("Task 1".to_string()),
        Task::new("Task 2".to_string()),
    ];

    let summary = display_task_summary(&tasks);

    assert!(summary.contains("Task Summary (2 tasks)"));
    assert!(summary.contains("Task 1"));
    assert!(summary.contains("Task 2"));
    assert!(summary.contains("Total Time:"));
    assert!(summary.contains("Running: "));
    assert!(summary.contains("Paused: "));
    assert!(summary.contains("Completed: "));
}

#[test]
fn test_display_task_summary_counts() {
    let mut tasks = vec![
        Task::new("Running Task".to_string()),
        Task::new("Paused Task".to_string()),
        Task::new("Completed Task".to_string()),
    ];

    // Pause the second task
    tasks[1].pause().unwrap();

    // Complete the third task
    tasks[2].complete().unwrap();

    let summary = display_task_summary(&tasks);

    assert!(summary.contains("Running: 1"));
    assert!(summary.contains("Paused: 1"));
    assert!(summary.contains("Completed: 1"));
}
