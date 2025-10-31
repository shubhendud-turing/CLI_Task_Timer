use predicates::prelude::*;
pub mod common;
use common::{fresh_test_command, test_command};

#[test]
fn test_cli_start_task() {
    let mut cmd = fresh_test_command("start_task");

    cmd.arg("start").arg("My Test Task");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Started task: 'My Test Task'"));
}

#[test]
fn test_cli_start_task_with_spaces() {
    let mut cmd = fresh_test_command("start_task_with_spaces");

    cmd.arg("start").arg("Task with spaces");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Started task: 'Task with spaces'"));
}

#[test]
fn test_cli_pause_without_active_task() {
    let mut cmd = fresh_test_command("pause_without_active_task");

    cmd.arg("pause");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No active task to operate on"));
}

#[test]
fn test_cli_resume_without_active_task() {
    let mut cmd = fresh_test_command("resume_without_active_task");

    cmd.arg("resume");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No active task to operate on"));
}

#[test]
fn test_cli_status_no_active_task() {
    let mut cmd = fresh_test_command("status_no_active_task");

    cmd.arg("status");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No active task"));
}

#[test]
fn test_cli_list_empty() {
    let mut cmd = fresh_test_command("list_empty");

    cmd.arg("list");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No tasks found"));
}

#[test]
fn test_cli_help() {
    let mut cmd = fresh_test_command("help");

    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "A CLI tool for tracking time spent on tasks",
        ))
        .stdout(predicate::str::contains("start"))
        .stdout(predicate::str::contains("pause"))
        .stdout(predicate::str::contains("resume"))
        .stdout(predicate::str::contains("status"))
        .stdout(predicate::str::contains("list"));
}

#[test]
fn test_cli_version() {
    let mut cmd = fresh_test_command("version");

    cmd.arg("--version");

    cmd.assert()
        .success()
        .stdout(predicate::str::is_match(r"task-timer \d+\.\d+\.\d+").unwrap());
}

#[test]
fn test_cli_invalid_command() {
    let mut cmd = fresh_test_command("invalid_command");

    cmd.arg("invalid-command");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("error: unrecognized subcommand"));
}

#[test]
fn test_cli_start_without_label() {
    let mut cmd = fresh_test_command("start_without_label");

    cmd.arg("start");

    cmd.assert().failure().stderr(predicate::str::contains(
        "error: the following required arguments were not provided",
    ));
}

#[test]
fn test_cli_complete_without_active_task() {
    let mut cmd = fresh_test_command("complete_without_active_task");

    cmd.arg("complete");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No active task to operate on"));
}

#[test]
fn test_cli_complete_running_task() {
    let test_name = "complete_running_task";

    // Start a task
    let mut cmd1 = fresh_test_command(test_name);
    cmd1.arg("start").arg("Task to complete");
    cmd1.assert().success();

    // Complete the task
    let mut cmd2 = test_command(test_name);
    cmd2.arg("complete");
    cmd2.assert().success().stdout(predicate::str::contains(
        "Completed task: 'Task to complete'",
    ));

    // Check status - should show no active task
    let mut cmd3 = test_command(test_name);
    cmd3.arg("status");
    cmd3.assert()
        .success()
        .stdout(predicate::str::contains("No active task"));

    // Check list - should show completed task
    let mut cmd4 = test_command(test_name);
    cmd4.arg("list");
    cmd4.assert()
        .success()
        .stdout(predicate::str::contains("Task to complete"))
        .stdout(predicate::str::contains("Completed"));
}

#[test]
fn test_cli_complete_paused_task() {
    let test_name = "complete_paused_task";

    // Start a task
    let mut cmd1 = fresh_test_command(test_name);
    cmd1.arg("start").arg("Paused task");
    cmd1.assert().success();

    // Pause the task
    let mut cmd2 = test_command(test_name);
    cmd2.arg("pause");
    cmd2.assert().success();

    // Complete the task
    let mut cmd3 = test_command(test_name);
    cmd3.arg("complete");
    cmd3.assert()
        .success()
        .stdout(predicate::str::contains("Completed task: 'Paused task'"));

    // Check status - should show no active task
    let mut cmd4 = test_command(test_name);
    cmd4.arg("status");
    cmd4.assert()
        .success()
        .stdout(predicate::str::contains("No active task"));
}
