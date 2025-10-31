use crate::task::{Task, TaskStatus};
use std::time::Duration;

/// Formats a duration into a human-readable string
pub(crate) fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}

/// Formats task status with appropriate symbols and colors (if terminal supports it)
pub(crate) fn format_status(status: &TaskStatus) -> String {
    match status {
        TaskStatus::Running => "🏃 Running".to_string(),
        TaskStatus::Paused => "⏸️  Paused".to_string(),
        TaskStatus::Completed => "✅ Completed".to_string(),
    }
}

/// Displays a single task with formatted information
pub(crate) fn display_task(task: &Task, index: Option<usize>) -> String {
    let status = format_status(&task.status);
    let duration = format_duration(task.total_duration());
    let created = task.created_at.format("%Y-%m-%d %H:%M:%S UTC");

    let prefix = if let Some(idx) = index {
        format!("{}. ", idx + 1)
    } else {
        String::new()
    };

    format!(
        "{}{} [{}] - {} (Created: {})",
        prefix, task.label, status, duration, created
    )
}

/// Displays current task status
pub(crate) fn display_current_status(task: Option<&Task>) -> String {
    match task {
        Some(task) => {
            let status = format_status(&task.status);
            let duration = format_duration(task.total_duration());

            format!("Current Task: {} [{}] - {}", task.label, status, duration)
        },
        None => "No active task".to_string(),
    }
}

/// Creates a summary of all tasks
pub(crate) fn display_task_summary(tasks: &[Task]) -> String {
    if tasks.is_empty() {
        return "No tasks found".to_string();
    }

    let mut output = String::new();
    output.push_str(&format!("Task Summary ({} tasks):\n", tasks.len()));
    output.push_str(&"=".repeat(40));
    output.push('\n');

    for (index, task) in tasks.iter().enumerate() {
        output.push_str(&display_task(task, Some(index)));
        output.push('\n');
    }

    // Calculate totals
    let total_duration: Duration = tasks.iter().map(|t| t.total_duration()).sum();

    let running_count = tasks.iter().filter(|t| t.is_running()).count();

    let paused_count = tasks.iter().filter(|t| t.is_paused()).count();

    let completed_count = tasks.iter().filter(|t| t.is_completed()).count();

    output.push('\n');
    output.push_str(&"=".repeat(40));
    output.push('\n');
    output.push_str(&format!(
        "Total Time: {}\n",
        format_duration(total_duration)
    ));
    output.push_str(&format!(
        "Running: {} | Paused: {} | Completed: {}",
        running_count, paused_count, completed_count
    ));

    output
}

#[cfg(test)]
mod display_tests;
