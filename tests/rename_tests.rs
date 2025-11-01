//! ==================== Rename Command Tests ====================
use predicates::prelude::*;
pub mod common;
use common::{fresh_test_command, test_command};

#[test]
fn test_cli_rename_completed_task() {
    let test_name = "rename_completed_task";

    // Create and complete a task
    let mut cmd1 = fresh_test_command(test_name);
    cmd1.arg("start").arg("Original Label");
    cmd1.assert().success();

    let mut cmd2 = test_command(test_name);
    cmd2.arg("complete");
    cmd2.assert().success();

    // Rename the task
    let mut cmd3 = test_command(test_name);
    cmd3.arg("rename").arg("1").arg("Updated Label");
    cmd3.assert().success().stdout(predicate::str::contains(
        "Task renamed from \"Original Label\" to \"Updated Label\"",
    ));

    // Verify the rename in list
    let mut cmd4 = test_command(test_name);
    cmd4.arg("list");
    cmd4.assert()
        .success()
        .stdout(predicate::str::contains("Updated Label"))
        .stdout(predicate::str::contains("Original Label").not());
}

#[test]
fn test_cli_rename_running_task() {
    let test_name = "rename_running_task";

    // Create a running task
    let mut cmd1 = fresh_test_command(test_name);
    cmd1.arg("start").arg("Running Task");
    cmd1.assert().success();

    // Rename the running task
    let mut cmd2 = test_command(test_name);
    cmd2.arg("rename").arg("1").arg("Renamed Running Task");
    cmd2.assert().success().stdout(predicate::str::contains(
        "Task renamed from \"Running Task\" to \"Renamed Running Task\"",
    ));

    // Verify task is still running with new name
    let mut cmd3 = test_command(test_name);
    cmd3.arg("status");
    cmd3.assert()
        .success()
        .stdout(predicate::str::contains("Renamed Running Task"))
        .stdout(predicate::str::contains("Running"));
}

#[test]
fn test_cli_rename_paused_task() {
    let test_name = "rename_paused_task";

    // Create and pause a task
    let mut cmd1 = fresh_test_command(test_name);
    cmd1.arg("start").arg("Paused Task");
    cmd1.assert().success();

    let mut cmd2 = test_command(test_name);
    cmd2.arg("pause");
    cmd2.assert().success();

    // Rename the paused task
    let mut cmd3 = test_command(test_name);
    cmd3.arg("rename").arg("1").arg("Renamed Paused Task");
    cmd3.assert().success().stdout(predicate::str::contains(
        "Task renamed from \"Paused Task\" to \"Renamed Paused Task\"",
    ));

    // Verify task is still paused with new name
    let mut cmd4 = test_command(test_name);
    cmd4.arg("status");
    cmd4.assert()
        .success()
        .stdout(predicate::str::contains("Renamed Paused Task"))
        .stdout(predicate::str::contains("Paused"));
}

#[test]
fn test_cli_rename_using_short_alias() {
    let test_name = "rename_short_alias";

    // Create a task
    let mut cmd1 = fresh_test_command(test_name);
    cmd1.arg("start").arg("Task to Rename");
    cmd1.assert().success();

    let mut cmd2 = test_command(test_name);
    cmd2.arg("complete");
    cmd2.assert().success();

    // Rename using short alias 'e'
    let mut cmd3 = test_command(test_name);
    cmd3.arg("e").arg("1").arg("Renamed with Alias");
    cmd3.assert()
        .success()
        .stdout(predicate::str::contains("Task renamed"));

    // Verify the rename
    let mut cmd4 = test_command(test_name);
    cmd4.arg("list");
    cmd4.assert()
        .success()
        .stdout(predicate::str::contains("Renamed with Alias"));
}

#[test]
fn test_cli_rename_invalid_index_zero() {
    let test_name = "rename_invalid_index_zero";

    // Create a task
    let mut cmd1 = fresh_test_command(test_name);
    cmd1.arg("start").arg("Test Task");
    cmd1.assert().success();

    let mut cmd2 = test_command(test_name);
    cmd2.arg("complete");
    cmd2.assert().success();

    // Try to rename with index 0
    let mut cmd3 = test_command(test_name);
    cmd3.arg("rename").arg("0").arg("New Label");
    cmd3.assert()
        .failure()
        .stderr(predicate::str::contains("must be greater than 0"));
}

#[test]
fn test_cli_rename_invalid_index_out_of_bounds() {
    let test_name = "rename_invalid_index_out_of_bounds";

    // Create a task
    let mut cmd1 = fresh_test_command(test_name);
    cmd1.arg("start").arg("Test Task");
    cmd1.assert().success();

    let mut cmd2 = test_command(test_name);
    cmd2.arg("complete");
    cmd2.assert().success();

    // Try to rename with index 99 (out of bounds)
    let mut cmd3 = test_command(test_name);
    cmd3.arg("rename").arg("99").arg("New Label");
    cmd3.assert()
        .failure()
        .stderr(predicate::str::contains("out of bounds"));
}

#[test]
fn test_cli_rename_empty_label() {
    let test_name = "rename_empty_label";

    // Create a task
    let mut cmd1 = fresh_test_command(test_name);
    cmd1.arg("start").arg("Test Task");
    cmd1.assert().success();

    let mut cmd2 = test_command(test_name);
    cmd2.arg("complete");
    cmd2.assert().success();

    // Try to rename with empty label
    let mut cmd3 = test_command(test_name);
    cmd3.arg("rename").arg("1").arg("");
    cmd3.assert()
        .failure()
        .stderr(predicate::str::contains("empty"));
}

#[test]
fn test_cli_rename_whitespace_only_label() {
    let test_name = "rename_whitespace_label";

    // Create a task
    let mut cmd1 = fresh_test_command(test_name);
    cmd1.arg("start").arg("Test Task");
    cmd1.assert().success();

    let mut cmd2 = test_command(test_name);
    cmd2.arg("complete");
    cmd2.assert().success();

    // Try to rename with whitespace-only label
    let mut cmd3 = test_command(test_name);
    cmd3.arg("rename").arg("1").arg("   ");
    cmd3.assert()
        .failure()
        .stderr(predicate::str::contains("empty"));
}

#[test]
fn test_cli_rename_empty_task_list() {
    let test_name = "rename_empty_task_list";

    // Try to rename with no tasks
    let mut cmd1 = fresh_test_command(test_name);
    cmd1.arg("rename").arg("1").arg("New Label");
    cmd1.assert()
        .failure()
        .stderr(predicate::str::contains("No tasks available"));
}

#[test]
fn test_cli_rename_multiple_tasks() {
    let test_name = "rename_multiple_tasks";

    // Create multiple tasks
    let mut cmd1 = fresh_test_command(test_name);
    cmd1.arg("start").arg("Task 1");
    cmd1.assert().success();

    let mut cmd2 = test_command(test_name);
    cmd2.arg("complete");
    cmd2.assert().success();

    let mut cmd3 = test_command(test_name);
    cmd3.arg("start").arg("Original Task Two");
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

    // Rename task 2 (middle task)
    let mut cmd7 = test_command(test_name);
    cmd7.arg("rename").arg("2").arg("Renamed Task Two");
    cmd7.assert().success();

    // Verify all tasks and that only task 2 was renamed
    let mut cmd8 = test_command(test_name);
    cmd8.arg("list");
    cmd8.assert()
        .success()
        .stdout(predicate::str::contains("Task 1"))
        .stdout(predicate::str::contains("Renamed Task Two"))
        .stdout(predicate::str::contains("Task 3"))
        .stdout(predicate::str::contains("Original Task Two").not());
}
