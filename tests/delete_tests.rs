//! ==================== Delete Command Tests ====================
use predicates::prelude::*;
pub mod common;
use common::{fresh_test_command, test_command};

#[test]
fn test_cli_delete_specific_task_by_index() {
    let test_name = "delete_specific_task_by_index";

    // Create multiple tasks
    let mut cmd1 = fresh_test_command(test_name);
    cmd1.arg("start").arg("Task 1");
    cmd1.assert().success();

    let mut cmd2 = test_command(test_name);
    cmd2.arg("complete");
    cmd2.assert().success();

    let mut cmd3 = test_command(test_name);
    cmd3.arg("start").arg("Task 2");
    cmd3.assert().success();

    let mut cmd4 = test_command(test_name);
    cmd4.arg("complete");
    cmd4.assert().success();

    let mut cmd5 = test_command(test_name);
    cmd5.arg("start").arg("Task 3");
    cmd5.assert().success();

    let mut cmd6 = test_command(test_name);
    cmd6.arg("complete");
    cmd6.assert().success();

    // List tasks to verify
    let mut cmd7 = test_command(test_name);
    cmd7.arg("list");
    cmd7.assert()
        .success()
        .stdout(predicate::str::contains("Task 1"))
        .stdout(predicate::str::contains("Task 2"))
        .stdout(predicate::str::contains("Task 3"));

    // Delete task 2 (index 2)
    let mut cmd8 = test_command(test_name);
    cmd8.arg("delete").arg("2");
    cmd8.assert().success().stdout(predicate::str::contains(
        "Task \"Task 2\" deleted successfully",
    ));

    // Verify task 2 is gone
    let mut cmd9 = test_command(test_name);
    cmd9.arg("list");
    cmd9.assert()
        .success()
        .stdout(predicate::str::contains("Task 1"))
        .stdout(predicate::str::contains("Task 3"))
        .stdout(predicate::str::contains("Task 2").not());
}

#[test]
fn test_cli_delete_specific_task_using_short_alias() {
    let test_name = "delete_task_short_alias";

    // Create a task
    let mut cmd1 = fresh_test_command(test_name);
    cmd1.arg("start").arg("Task to delete with alias");
    cmd1.assert().success();

    let mut cmd2 = test_command(test_name);
    cmd2.arg("complete");
    cmd2.assert().success();

    // Delete using short alias 'd'
    let mut cmd3 = test_command(test_name);
    cmd3.arg("d").arg("1");
    cmd3.assert()
        .success()
        .stdout(predicate::str::contains("deleted successfully"));

    // Verify task is gone
    let mut cmd4 = test_command(test_name);
    cmd4.arg("list");
    cmd4.assert()
        .success()
        .stdout(predicate::str::contains("No tasks found"));
}

#[test]
fn test_cli_delete_all_completed_tasks() {
    let test_name = "delete_all_completed_tasks";

    // Create mix of completed and active tasks
    let mut cmd1 = fresh_test_command(test_name);
    cmd1.arg("start").arg("Completed Task 1");
    cmd1.assert().success();

    let mut cmd2 = test_command(test_name);
    cmd2.arg("complete");
    cmd2.assert().success();

    let mut cmd3 = test_command(test_name);
    cmd3.arg("start").arg("Completed Task 2");
    cmd3.assert().success();

    let mut cmd4 = test_command(test_name);
    cmd4.arg("complete");
    cmd4.assert().success();

    let mut cmd5 = test_command(test_name);
    cmd5.arg("start").arg("Active Task");
    cmd5.assert().success();

    // Delete all completed tasks
    let mut cmd6 = test_command(test_name);
    cmd6.arg("delete").arg("--completed");
    cmd6.assert().success().stdout(predicate::str::contains(
        "2 completed task(s) deleted successfully",
    ));

    // Verify only active task remains
    let mut cmd7 = test_command(test_name);
    cmd7.arg("list");
    cmd7.assert()
        .success()
        .stdout(predicate::str::contains("Active Task"))
        .stdout(predicate::str::contains("Completed Task 1").not())
        .stdout(predicate::str::contains("Completed Task 2").not());
}

#[test]
fn test_cli_delete_all_completed_tasks_using_short_alias() {
    let test_name = "delete_completed_short_alias";

    // Create completed tasks
    let mut cmd1 = fresh_test_command(test_name);
    cmd1.arg("start").arg("Task 1");
    cmd1.assert().success();

    let mut cmd2 = test_command(test_name);
    cmd2.arg("complete");
    cmd2.assert().success();

    // Delete using short alias 'd' with --completed
    let mut cmd3 = test_command(test_name);
    cmd3.arg("d").arg("--completed");
    cmd3.assert().success().stdout(predicate::str::contains(
        "1 completed task(s) deleted successfully",
    ));

    // Verify all tasks are gone
    let mut cmd4 = test_command(test_name);
    cmd4.arg("list");
    cmd4.assert()
        .success()
        .stdout(predicate::str::contains("No tasks found"));
}

#[test]
fn test_cli_delete_invalid_index_out_of_bounds() {
    let test_name = "delete_invalid_index";

    // Create one task
    let mut cmd1 = fresh_test_command(test_name);
    cmd1.arg("start").arg("Only Task");
    cmd1.assert().success();

    let mut cmd2 = test_command(test_name);
    cmd2.arg("complete");
    cmd2.assert().success();

    // Try to delete index 5 (out of bounds)
    let mut cmd3 = test_command(test_name);
    cmd3.arg("delete").arg("5");
    cmd3.assert()
        .failure()
        .stderr(predicate::str::contains("out of bounds"));

    // Try to delete index 0 (invalid)
    let mut cmd4 = test_command(test_name);
    cmd4.arg("delete").arg("0");
    cmd4.assert()
        .failure()
        .stderr(predicate::str::contains("must be greater than 0"));
}

#[test]
fn test_cli_delete_active_running_task() {
    let test_name = "delete_active_running_task";

    // Create and start a task (running)
    let mut cmd1 = fresh_test_command(test_name);
    cmd1.arg("start").arg("Running Task");
    cmd1.assert().success();

    // Try to delete the running task
    let mut cmd2 = test_command(test_name);
    cmd2.arg("delete").arg("1");
    cmd2.assert()
        .failure()
        .stderr(predicate::str::contains("currently running"));
}

#[test]
fn test_cli_delete_active_paused_task() {
    let test_name = "delete_active_paused_task";

    // Create, start, and pause a task
    let mut cmd1 = fresh_test_command(test_name);
    cmd1.arg("start").arg("Paused Task");
    cmd1.assert().success();

    let mut cmd2 = test_command(test_name);
    cmd2.arg("pause");
    cmd2.assert().success();

    // Try to delete the paused task
    let mut cmd3 = test_command(test_name);
    cmd3.arg("delete").arg("1");
    cmd3.assert()
        .failure()
        .stderr(predicate::str::contains("currently paused"));
}

#[test]
fn test_cli_delete_empty_task_list() {
    let test_name = "delete_empty_task_list";

    // Try to delete from empty list
    let mut cmd1 = fresh_test_command(test_name);
    cmd1.arg("delete").arg("1");
    cmd1.assert()
        .failure()
        .stderr(predicate::str::contains("No tasks available to delete"));
}

#[test]
fn test_cli_delete_no_completed_tasks() {
    let test_name = "delete_no_completed_tasks";

    // Create only running task
    let mut cmd1 = fresh_test_command(test_name);
    cmd1.arg("start").arg("Running Task");
    cmd1.assert().success();

    // Try to delete completed tasks (none exist)
    let mut cmd2 = test_command(test_name);
    cmd2.arg("delete").arg("--completed");
    cmd2.assert()
        .success()
        .stdout(predicate::str::contains("No completed tasks to delete"));

    // Verify running task still exists
    let mut cmd3 = test_command(test_name);
    cmd3.arg("status");
    cmd3.assert()
        .success()
        .stdout(predicate::str::contains("Running Task"));
}

#[test]
fn test_cli_delete_updates_active_task_index() {
    let test_name = "delete_updates_active_index";

    // Create three tasks
    let mut cmd1 = fresh_test_command(test_name);
    cmd1.arg("start").arg("Task 1");
    cmd1.assert().success();

    let mut cmd2 = test_command(test_name);
    cmd2.arg("complete");
    cmd2.assert().success();

    let mut cmd3 = test_command(test_name);
    cmd3.arg("start").arg("Task 2");
    cmd3.assert().success();

    let mut cmd4 = test_command(test_name);
    cmd4.arg("complete");
    cmd4.assert().success();

    let mut cmd5 = test_command(test_name);
    cmd5.arg("start").arg("Task 3 Active");
    cmd5.assert().success();

    // Delete Task 1 (before the active task)
    let mut cmd6 = test_command(test_name);
    cmd6.arg("delete").arg("1");
    cmd6.assert().success();

    // Verify active task is still accessible
    let mut cmd7 = test_command(test_name);
    cmd7.arg("status");
    cmd7.assert()
        .success()
        .stdout(predicate::str::contains("Task 3 Active"));

    // Should be able to pause the active task
    let mut cmd8 = test_command(test_name);
    cmd8.arg("pause");
    cmd8.assert().success();
}
