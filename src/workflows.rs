/// Workflow test demonstrating the core functionality
/// This tests the start and pause features in a single process
use crate::task::{TaskManager, TaskStatus};
use std::thread;
use std::time::Duration;

#[test]
fn test_complete_workflow_start_and_pause() {
    let mut manager = TaskManager::new();

    // Step 1: Start a new task
    let task_index = manager
        .start_task("Complete CLI Implementation".to_string())
        .unwrap();
    assert_eq!(task_index, 0);

    // Verify task is running
    let current_task = manager.current_task().unwrap();
    assert_eq!(current_task.label, "Complete CLI Implementation");
    assert_eq!(current_task.status, TaskStatus::Running);
    assert!(current_task.is_running());

    // Step 2: Let the task run for a small amount of time
    thread::sleep(Duration::from_millis(50));

    // Step 3: Pause the task
    let pause_result = manager.pause_current_task();
    assert!(pause_result.is_ok());

    // Verify task is paused and has accumulated time
    let current_task = manager.current_task().unwrap();
    assert_eq!(current_task.status, TaskStatus::Paused);
    assert!(current_task.is_paused());
    assert!(current_task.total_duration() > Duration::ZERO);

    println!("✅ Workflow test passed: Start task → Let it run → Pause task");
    println!(
        "   Task '{}' accumulated {} seconds",
        current_task.label,
        current_task.total_duration().as_secs_f64()
    );
}

#[test]
fn test_multiple_tasks_workflow() {
    let mut manager = TaskManager::new();

    // Start first task
    manager.start_task("Task 1: Design".to_string()).unwrap();
    thread::sleep(Duration::from_millis(20));

    // Start second task (should auto-pause first task)
    manager
        .start_task("Task 2: Implementation".to_string())
        .unwrap();
    thread::sleep(Duration::from_millis(20));

    // Verify second task is active and first is paused
    let current_task = manager.current_task().unwrap();
    assert_eq!(current_task.label, "Task 2: Implementation");
    assert!(current_task.is_running());

    let all_tasks = manager.all_tasks();
    assert_eq!(all_tasks.len(), 2);

    // First task should be paused
    assert!(all_tasks[0].is_paused());
    assert_eq!(all_tasks[0].label, "Task 1: Design");

    // Second task should be running
    assert!(all_tasks[1].is_running());
    assert_eq!(all_tasks[1].label, "Task 2: Implementation");

    // Pause current task
    manager.pause_current_task().unwrap();

    // Both tasks should now be paused
    let all_tasks = manager.all_tasks();
    assert!(all_tasks[0].is_paused());
    assert!(all_tasks[1].is_paused());

    // Both should have accumulated some time
    assert!(all_tasks[0].total_duration() > Duration::ZERO);
    assert!(all_tasks[1].total_duration() > Duration::ZERO);

    println!("✅ Multiple tasks workflow passed");
    println!(
        "   Task 1 time: {:.3}s",
        all_tasks[0].total_duration().as_secs_f64()
    );
    println!(
        "   Task 2 time: {:.3}s",
        all_tasks[1].total_duration().as_secs_f64()
    );
}

#[test]
fn test_error_handling_workflow() {
    let mut manager = TaskManager::new();

    // Try to pause without any task - should fail
    let result = manager.pause_current_task();
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "No active task to operate on"
    );

    // Try to resume without any task - should fail
    let result = manager.resume_current_task();
    assert!(result.is_err());

    // Start a task
    manager.start_task("Test Task".to_string()).unwrap();

    // Try to resume a running task - should fail
    let result = manager.resume_current_task();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Task is already running");

    // Pause the task
    manager.pause_current_task().unwrap();

    // Try to pause an already paused task - should fail
    let result = manager.pause_current_task();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Task is already paused");

    println!("✅ Error handling workflow passed");
}
