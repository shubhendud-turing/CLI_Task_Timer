use super::*;
use clap::CommandFactory;

#[test]
fn test_cli_parsing_start_command() {
    let args = vec!["task-timer", "start", "My Task"];
    let cli = Cli::try_parse_from(args).unwrap();

    match cli.command {
        Commands::Start { label } => {
            assert_eq!(label, "My Task");
        },
        _ => panic!("Expected Start command"),
    }
}

#[test]
fn test_cli_parsing_pause_command() {
    let args = vec!["task-timer", "pause"];
    let cli = Cli::try_parse_from(args).unwrap();

    matches!(cli.command, Commands::Pause);
}

#[test]
fn test_cli_parsing_resume_command() {
    let args = vec!["task-timer", "resume"];
    let cli = Cli::try_parse_from(args).unwrap();

    matches!(cli.command, Commands::Resume);
}

#[test]
fn test_cli_parsing_status_command() {
    let args = vec!["task-timer", "status"];
    let cli = Cli::try_parse_from(args).unwrap();

    matches!(cli.command, Commands::Status);
}

#[test]
fn test_cli_parsing_list_command() {
    let args = vec!["task-timer", "list"];
    let cli = Cli::try_parse_from(args).unwrap();

    matches!(cli.command, Commands::List);
}

#[test]
fn test_cli_help_generation() {
    let mut cmd = Cli::command();
    let help = cmd.render_help();
    let help_str = help.to_string();

    // The CLI description text is "A CLI tool for tracking time spent on tasks"
    assert!(help_str.contains("CLI tool for tracking"));
    assert!(help_str.contains("start"));
    assert!(help_str.contains("pause"));
    assert!(help_str.contains("resume"));
}

#[test]
fn test_start_command_requires_label() {
    let args = vec!["task-timer", "start"];
    let result = Cli::try_parse_from(args);

    assert!(result.is_err());
}

#[test]
fn test_start_command_with_multi_word_label() {
    let args = vec!["task-timer", "start", "My", "Complex", "Task", "Name"];
    let result = Cli::try_parse_from(args);

    // This should fail because clap expects a single argument for label
    // User would need to quote: "My Complex Task Name"
    assert!(result.is_err());
}

#[test]
fn test_start_command_with_quoted_label() {
    let args = vec!["task-timer", "start", "My Complex Task Name"];
    let cli = Cli::try_parse_from(args).unwrap();

    match cli.command {
        Commands::Start { label } => {
            assert_eq!(label, "My Complex Task Name");
        },
        _ => panic!("Expected Start command"),
    }
}

#[test]
fn test_command_names() {
    assert_eq!(
        Commands::Start { label: "test".to_string() }.name(),
        "start"
    );
    assert_eq!(Commands::Pause.name(), "pause");
    assert_eq!(Commands::Resume.name(), "resume");
    assert_eq!(Commands::Status.name(), "status");
    assert_eq!(Commands::List.name(), "list");
    assert_eq!(Commands::Complete.name(), "complete");
}
