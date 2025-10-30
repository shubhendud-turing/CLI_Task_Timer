use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

#[test]
fn test_cli_start_task() {
    let mut cmd = cargo_bin_cmd!("tt");

    cmd.arg("start").arg("My Test Task");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Started task: 'My Test Task'"));
}

#[test]
fn test_cli_start_task_with_spaces() {
    let mut cmd = cargo_bin_cmd!("tt");

    cmd.arg("start").arg("Task with spaces");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Started task: 'Task with spaces'"));
}

#[test]
fn test_cli_pause_without_active_task() {
    let mut cmd = cargo_bin_cmd!("tt");

    cmd.arg("pause");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No active task to operate on"));
}

#[test]
fn test_cli_resume_without_active_task() {
    let mut cmd = cargo_bin_cmd!("tt");

    cmd.arg("resume");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No active task to operate on"));
}

#[test]
fn test_cli_status_no_active_task() {
    let mut cmd = cargo_bin_cmd!("tt");

    cmd.arg("status");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No active task"));
}

#[test]
fn test_cli_list_empty() {
    let mut cmd = cargo_bin_cmd!("tt");

    cmd.arg("list");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No tasks found"));
}

#[test]
fn test_cli_help() {
    let mut cmd = cargo_bin_cmd!("tt");

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
    let mut cmd = cargo_bin_cmd!("tt");

    cmd.arg("--version");

    cmd.assert()
        .success()
        .stdout(predicate::str::is_match(r"task-timer \d+\.\d+\.\d+").unwrap());
}

#[test]
fn test_cli_invalid_command() {
    let mut cmd = cargo_bin_cmd!("tt");

    cmd.arg("invalid-command");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("error: unrecognized subcommand"));
}

#[test]
fn test_cli_start_without_label() {
    let mut cmd = cargo_bin_cmd!("tt");

    cmd.arg("start");

    cmd.assert().failure().stderr(predicate::str::contains(
        "error: the following required arguments were not provided",
    ));
}

// Note: These workflow tests would need a way to persist state between commands
// For now, they demonstrate the expected behavior in a single process
mod workflow_tests {
    // Since the current implementation doesn't persist state,
    // we'll test the workflows in the unit tests instead
    // These integration tests focus on individual command validation
}
