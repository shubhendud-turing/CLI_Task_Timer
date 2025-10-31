use assert_cmd::cargo::cargo_bin_cmd;
use std::{env, fs};

/// Create a command with test-specific config directory that persists across calls within the same test
pub fn test_command(test_name: &str) -> assert_cmd::Command {
    let temp_dir = env::temp_dir().join("tt_tests").join(test_name);

    // Only create/clean directory if it doesn't exist
    // This ensures persistence within the same test
    if !temp_dir.exists() {
        fs::create_dir_all(&temp_dir).unwrap();
    }

    let mut cmd = cargo_bin_cmd!("tt");
    cmd.env("TT_CONFIG_DIR", temp_dir);
    cmd
}

/// Create a FRESH command that cleans the test directory first
pub fn fresh_test_command(test_name: &str) -> assert_cmd::Command {
    cleanup_test_dir(test_name);
    test_command(test_name)
}

/// Clean up a specific test directory
pub fn cleanup_test_dir(test_name: &str) {
    let temp_dir = env::temp_dir().join("tt_tests").join(test_name);
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir).ok(); // Ignore errors
    }
}

/// Clean up ALL test directories (call this in test setup/teardown)
pub fn cleanup_all_test_dirs() {
    let base_dir = env::temp_dir().join("tt_tests");
    if base_dir.exists() {
        fs::remove_dir_all(&base_dir).ok(); // Ignore errors
    }
}

/// Helper to run a test with automatic cleanup afterward
pub fn with_test_cleanup<F>(test_name: &str, test_fn: F)
where
    F: FnOnce(assert_cmd::Command),
{
    let cmd = test_command(test_name);
    test_fn(cmd);
    cleanup_test_dir(test_name); // Always cleanup after test
}
