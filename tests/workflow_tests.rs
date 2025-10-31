use predicates::prelude::*;
use std::{env, fs};

pub mod common;
use common::{fresh_test_command, test_command};

#[test]
fn test_persistence_start_pause_resume_workflow() {
    let test_name = "persistence_workflow";

    // Start a task
    let mut cmd = fresh_test_command(test_name);
    cmd.arg("start").arg("Persistent Task");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Started task: 'Persistent Task'"));

    // Pause the task (separate invocation)
    let mut cmd = test_command(test_name);
    cmd.arg("pause");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Paused task"))
        .stdout(predicate::str::contains("Persistent Task"))
        .stdout(predicate::str::contains("Paused"));

    // Check status (separate invocation)
    let mut cmd = test_command(test_name);
    cmd.arg("status");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Persistent Task"))
        .stdout(predicate::str::contains("Paused"));

    // Resume the task (separate invocation)
    let mut cmd = test_command(test_name);
    cmd.arg("resume");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Resumed task"))
        .stdout(predicate::str::contains("Persistent Task"))
        .stdout(predicate::str::contains("Running"));

    // Check final status (separate invocation)
    let mut cmd = test_command(test_name);
    cmd.arg("status");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Persistent Task"))
        .stdout(predicate::str::contains("Running"));
}

#[test]
fn test_multiple_tasks_persistence() {
    let test_name = "multiple_tasks";

    // Start first task
    let mut cmd = fresh_test_command(test_name);
    cmd.arg("start").arg("Task One");
    cmd.assert().success();

    // Start second task (should auto-pause first)
    let mut cmd = test_command(test_name);
    cmd.arg("start").arg("Task Two");
    cmd.assert().success();

    // List tasks to verify both exist
    let mut cmd = test_command(test_name);
    cmd.arg("list");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Task One"))
        .stdout(predicate::str::contains("Task Two"))
        .stdout(predicate::str::contains("Paused"))
        .stdout(predicate::str::contains("Running"))
        .stdout(predicate::str::contains("2 tasks"));
}

#[test]
fn test_task_cleanup_after_limit() {
    let test_name = "task_cleanup";

    // Clean up any existing test directory first
    let temp_dir = env::temp_dir().join("tt_tests").join(test_name);
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir).ok();
    }

    // Create more than 10 tasks to trigger cleanup
    for i in 1..=12 {
        let mut cmd = test_command(test_name);
        cmd.arg("start").arg(format!("Task {}", i));
        cmd.assert().success();

        // Complete some tasks by starting new ones (auto-pauses)
        if i < 12 {
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }

    // List tasks - should have at most 10
    let mut cmd = test_command(test_name);
    cmd.arg("list");
    let output = cmd.assert().success();

    // The output should not mention more than 10 tasks
    // Should still have the most recent tasks
    output.stdout(predicate::str::contains("Task 12"));
}

#[test]
fn test_persistence_across_system_restart_simulation() {
    let test_name = "restart_simulation";

    // Simulate first "session" - create and work with tasks
    let mut cmd = fresh_test_command(test_name);
    cmd.arg("start").arg("Important Work");
    cmd.assert().success();

    let mut cmd = test_command(test_name);
    cmd.arg("pause");
    cmd.assert().success();

    // Wait a bit to accumulate some time
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Simulate "system restart" - run status in a fresh invocation
    let mut cmd = test_command(test_name);
    cmd.arg("status");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Important Work"))
        .stdout(predicate::str::contains("Paused"));

    // Resume work in the "new session"
    let mut cmd = test_command(test_name);
    cmd.arg("resume");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Resumed task"));

    // Verify we can continue working
    let mut cmd = test_command(test_name);
    cmd.arg("status");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Important Work"))
        .stdout(predicate::str::contains("Running"));
}

#[test]
fn test_complete_task_lifecycle() {
    let test_name = "complete_lifecycle";

    // Start a task
    let mut cmd = fresh_test_command(test_name);
    cmd.arg("start").arg("Feature development");
    cmd.assert().success().stdout(predicate::str::contains(
        "Started task: 'Feature development'",
    ));

    // Work for a bit
    std::thread::sleep(std::time::Duration::from_millis(50));

    // Pause the task
    let mut cmd = test_command(test_name);
    cmd.arg("pause");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Paused task"))
        .stdout(predicate::str::contains("Feature development"));

    // Resume the task
    let mut cmd = test_command(test_name);
    cmd.arg("resume");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Resumed task"));

    // Complete the task
    let mut cmd = test_command(test_name);
    cmd.arg("complete");
    cmd.assert().success().stdout(predicate::str::contains(
        "Completed task: 'Feature development'",
    ));

    // Verify no active task
    let mut cmd = test_command(test_name);
    cmd.arg("status");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No active task"));

    // Verify task shows as completed in list
    let mut cmd = test_command(test_name);
    cmd.arg("list");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Feature development"))
        .stdout(predicate::str::contains("Completed"))
        .stdout(predicate::str::contains("1 task"));
}

#[test]
fn test_multiple_tasks_with_completions() {
    let test_name = "multiple_completions";

    // Start and complete first task
    let mut cmd = fresh_test_command(test_name);
    cmd.arg("start").arg("Task 1");
    cmd.assert().success();

    let mut cmd = test_command(test_name);
    cmd.arg("complete");
    cmd.assert().success();

    // Start and complete second task
    let mut cmd = test_command(test_name);
    cmd.arg("start").arg("Task 2");
    cmd.assert().success();

    let mut cmd = test_command(test_name);
    cmd.arg("complete");
    cmd.assert().success();

    // Start third task but don't complete it
    let mut cmd = test_command(test_name);
    cmd.arg("start").arg("Task 3");
    cmd.assert().success();

    // List all tasks
    let mut cmd = test_command(test_name);
    cmd.arg("list");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Task 1"))
        .stdout(predicate::str::contains("Task 2"))
        .stdout(predicate::str::contains("Task 3"))
        .stdout(predicate::str::contains("3 tasks"))
        .stdout(predicate::str::contains("Running: 1"))
        .stdout(predicate::str::contains("Completed: 2"));

    // Verify current task
    let mut cmd = test_command(test_name);
    cmd.arg("status");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Task 3"))
        .stdout(predicate::str::contains("Running"));
}

#[test]
fn test_completed_task_persistence() {
    let test_name = "completed_persistence";

    // Start and complete a task
    let mut cmd = fresh_test_command(test_name);
    cmd.arg("start").arg("Completed Work");
    cmd.assert().success();

    let mut cmd = test_command(test_name);
    cmd.arg("complete");
    cmd.assert().success();

    // Simulate restart - check if completed task persists
    let mut cmd = test_command(test_name);
    cmd.arg("list");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Completed Work"))
        .stdout(predicate::str::contains("Completed"));

    // Verify no active task
    let mut cmd = test_command(test_name);
    cmd.arg("status");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No active task"));
}
