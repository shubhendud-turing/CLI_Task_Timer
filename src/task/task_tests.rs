use super::*;
use std::thread;
use std::time::Duration as StdDuration;

#[test]
fn test_new_task_creation() {
    let task = Task::new("Test Task".to_string());

    assert_eq!(task.label, "Test Task");
    assert_eq!(task.status, TaskStatus::Running);
    assert!(task.started_at.is_some());
    assert_eq!(task.accumulated_duration, Duration::ZERO);
    assert!(task.is_running());
    assert!(!task.is_paused());
    assert!(!task.is_completed());
}

#[test]
fn test_task_pause() {
    let mut task = Task::new("Test Task".to_string());

    // Small delay to ensure measurable duration
    thread::sleep(StdDuration::from_millis(10));

    let result = task.pause();
    assert!(result.is_ok());
    assert_eq!(task.status, TaskStatus::Paused);
    assert!(task.started_at.is_none());
    assert!(task.accumulated_duration > Duration::ZERO);
    assert!(task.is_paused());
    assert!(!task.is_running());
}

#[test]
fn test_task_pause_already_paused() {
    let mut task = Task::new("Test Task".to_string());
    task.pause().unwrap();

    let result = task.pause();
    assert!(result.is_err());
    match result.unwrap_err() {
        TaskError::TaskAlreadyPaused => {},
        _ => panic!("Expected TaskAlreadyPaused error"),
    }
}

#[test]
fn test_task_resume() {
    let mut task = Task::new("Test Task".to_string());
    task.pause().unwrap();

    let result = task.resume();
    assert!(result.is_ok());
    assert_eq!(task.status, TaskStatus::Running);
    assert!(task.started_at.is_some());
    assert!(task.is_running());
    assert!(!task.is_paused());
}

#[test]
fn test_task_resume_already_running() {
    let mut task = Task::new("Test Task".to_string());

    let result = task.resume();
    assert!(result.is_err());
    match result.unwrap_err() {
        TaskError::TaskAlreadyRunning => {},
        _ => panic!("Expected TaskAlreadyRunning error"),
    }
}

#[test]
fn test_task_total_duration() {
    let mut task = Task::new("Test Task".to_string());

    // Run for a bit
    thread::sleep(StdDuration::from_millis(10));
    task.pause().unwrap();

    let duration_after_pause = task.total_duration();
    assert!(duration_after_pause > Duration::ZERO);

    // Resume and run again
    task.resume().unwrap();
    thread::sleep(StdDuration::from_millis(10));

    let duration_while_running = task.total_duration();
    assert!(duration_while_running > duration_after_pause);
}

#[test]
fn test_task_manager_start_task() {
    let mut manager = TaskManager::new();

    let task_index = manager.start_task("First Task".to_string()).unwrap();
    assert_eq!(task_index, 0);
    assert_eq!(manager.task_count(), 1);
    assert!(manager.has_running_task());

    let current_task = manager.current_task().unwrap();
    assert_eq!(current_task.label, "First Task");
    assert!(current_task.is_running());
}

#[test]
fn test_task_manager_start_multiple_tasks() {
    let mut manager = TaskManager::new();

    // Start first task
    manager.start_task("First Task".to_string()).unwrap();
    thread::sleep(StdDuration::from_millis(10));

    // Start second task (should pause the first)
    manager.start_task("Second Task".to_string()).unwrap();

    assert_eq!(manager.task_count(), 2);

    // First task should be paused
    assert!(manager.all_tasks()[0].is_paused());

    // Second task should be running
    let current_task = manager.current_task().unwrap();
    assert_eq!(current_task.label, "Second Task");
    assert!(current_task.is_running());
}

#[test]
fn test_task_manager_pause_current() {
    let mut manager = TaskManager::new();
    manager.start_task("Test Task".to_string()).unwrap();

    thread::sleep(StdDuration::from_millis(10));

    let result = manager.pause_current_task();
    assert!(result.is_ok());

    let current_task = manager.current_task().unwrap();
    assert!(current_task.is_paused());
    assert!(!manager.has_running_task());
}

#[test]
fn test_task_manager_pause_no_active_task() {
    let mut manager = TaskManager::new();

    let result = manager.pause_current_task();
    assert!(result.is_err());
    match result.unwrap_err() {
        TaskError::NoActiveTask => {},
        _ => panic!("Expected NoActiveTask error"),
    }
}

#[test]
fn test_task_manager_resume_current() {
    let mut manager = TaskManager::new();
    manager.start_task("Test Task".to_string()).unwrap();
    manager.pause_current_task().unwrap();

    let result = manager.resume_current_task();
    assert!(result.is_ok());

    let current_task = manager.current_task().unwrap();
    assert!(current_task.is_running());
    assert!(manager.has_running_task());
}

#[test]
fn test_task_complete() {
    let mut task = Task::new("Test Task".to_string());
    thread::sleep(StdDuration::from_millis(10));

    let result = task.complete();
    assert!(result.is_ok());
    assert!(task.is_completed());
    assert!(task.total_duration() > Duration::ZERO);

    // Should not be able to pause/resume completed task
    let pause_result = task.pause();
    assert!(pause_result.is_err());
    match pause_result.unwrap_err() {
        TaskError::TaskCompleted => {},
        _ => panic!("Expected TaskCompleted error"),
    }

    let resume_result = task.resume();
    assert!(resume_result.is_err());
    match resume_result.unwrap_err() {
        TaskError::TaskCompleted => {},
        _ => panic!("Expected TaskCompleted error"),
    }
}

#[test]
fn test_serialize_deserialize_task_manager() {
    let mut manager = TaskManager::new();
    let _task_id = manager.start_task("Test Task".to_string()).unwrap();
    manager.pause_current_task().unwrap();

    // Serialize to JSON
    let json = serde_json::to_string(&manager).unwrap();
    assert!(json.contains("Test Task"));
    assert!(json.contains("Paused"));

    // Deserialize from JSON
    let deserialized: TaskManager = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.tasks.len(), 1);
    assert_eq!(deserialized.active_task_index, Some(0));
    assert_eq!(deserialized.tasks[0].label, "Test Task");
    assert!(deserialized.tasks[0].is_paused());
}

#[test]
fn test_cleanup_old_tasks() {
    let mut manager = TaskManager::new();

    // Create 15 tasks (more than the 10 limit)
    for i in 0..15 {
        let _task_id = manager.start_task(format!("Task {}", i)).unwrap();
        // Complete old tasks by directly modifying tasks (simulating completed state)
        if i < 10 {
            // Access task directly for testing purposes
            if let Some(index) = manager.active_task_index {
                manager.tasks[index].status = TaskStatus::Completed;
                manager.active_task_index = None;
            }
        }
    }

    assert_eq!(manager.tasks.len(), 15);

    // Run cleanup
    manager.cleanup_old_tasks();

    // Should have at most 10 tasks
    assert!(manager.tasks.len() <= 10);

    // Should preserve the most recent active/incomplete tasks
    let has_recent_tasks = manager
        .tasks
        .iter()
        .any(|task| task.label.contains("Task 14") || task.label.contains("Task 13"));
    assert!(has_recent_tasks);
}

#[test]
fn test_cleanup_preserves_active_task() {
    let mut manager = TaskManager::new();

    // Create many completed tasks
    for i in 0..12 {
        let _task_id = manager.start_task(format!("Completed Task {}", i)).unwrap();
        // Simulate completion by setting status directly
        if let Some(index) = manager.active_task_index {
            manager.tasks[index].status = TaskStatus::Completed;
            manager.active_task_index = None;
        }
    }

    // Create one active task
    let _active_id = manager
        .start_task("Important Active Task".to_string())
        .unwrap();

    assert_eq!(manager.tasks.len(), 13);

    // Run cleanup
    manager.cleanup_old_tasks();

    // Should still have the active task
    assert!(manager.active_task_index.is_some());
    let current_task = manager.current_task().unwrap();
    assert_eq!(current_task.label, "Important Active Task");
    assert!(manager.tasks.len() <= 10);
}

#[test]
fn test_get_config_path() {
    let path_result = TaskManager::get_config_path();
    assert!(path_result.is_ok());

    let path = path_result.unwrap();
    assert!(path.to_string_lossy().contains("tt"));
    assert!(path.to_string_lossy().ends_with("tasks.json"));
}

#[test]
fn test_complete_current_task_running() {
    let mut manager = TaskManager::new();
    manager.start_task("Test Task".to_string()).unwrap();
    thread::sleep(StdDuration::from_millis(10));

    let result = manager.complete_current_task();
    assert!(result.is_ok());

    // Should have no active task after completion
    assert!(manager.current_task().is_none());
    assert_eq!(manager.active_task_index, None);

    // Task should be completed
    assert_eq!(manager.tasks.len(), 1);
    assert!(manager.tasks[0].is_completed());
    assert!(manager.tasks[0].total_duration() > Duration::ZERO);
}

#[test]
fn test_complete_current_task_paused() {
    let mut manager = TaskManager::new();
    manager.start_task("Test Task".to_string()).unwrap();
    thread::sleep(StdDuration::from_millis(10));
    manager.pause_current_task().unwrap();

    let result = manager.complete_current_task();
    assert!(result.is_ok());

    // Should have no active task after completion
    assert!(manager.current_task().is_none());
    assert_eq!(manager.active_task_index, None);

    // Task should be completed
    assert_eq!(manager.tasks.len(), 1);
    assert!(manager.tasks[0].is_completed());
}

#[test]
fn test_complete_current_task_no_active() {
    let mut manager = TaskManager::new();

    let result = manager.complete_current_task();
    assert!(result.is_err());
    match result.unwrap_err() {
        TaskError::NoActiveTask => {},
        _ => panic!("Expected NoActiveTask error"),
    }
}

#[test]
fn test_complete_task_cannot_be_resumed() {
    let mut manager = TaskManager::new();
    manager.start_task("Test Task".to_string()).unwrap();
    manager.complete_current_task().unwrap();

    // Task is completed and no longer active
    // Trying to resume should fail since there's no active task
    let result = manager.resume_current_task();
    assert!(result.is_err());
    match result.unwrap_err() {
        TaskError::NoActiveTask => {},
        _ => panic!("Expected NoActiveTask error"),
    }
}

#[test]
fn test_multiple_tasks_with_completion() {
    let mut manager = TaskManager::new();

    // Start and complete first task
    manager.start_task("Task 1".to_string()).unwrap();
    thread::sleep(StdDuration::from_millis(10));
    manager.complete_current_task().unwrap();

    // Start and complete second task
    manager.start_task("Task 2".to_string()).unwrap();
    thread::sleep(StdDuration::from_millis(10));
    manager.complete_current_task().unwrap();

    // Start third task but don't complete it
    manager.start_task("Task 3".to_string()).unwrap();

    // Should have 3 tasks total
    assert_eq!(manager.tasks.len(), 3);

    // First two should be completed
    assert!(manager.tasks[0].is_completed());
    assert!(manager.tasks[1].is_completed());

    // Third should be running
    assert!(manager.tasks[2].is_running());
    assert!(manager.current_task().is_some());
    assert_eq!(manager.current_task().unwrap().label, "Task 3");
}
