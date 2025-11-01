# Task Timer (`tt`)

A fast, lightweight command-line tool for tracking time spent on tasks. Perfect for developers, freelancers, and anyone who needs to monitor their work sessions.

## Features

- ⏱️ **Start/Pause/Resume Tasks**: Create labeled tasks and pause/resume timing
- ✅ **Complete Tasks**: Mark tasks as completed when finished
- 🏃 **Real-time Tracking**: Accurate time measurement with nanosecond precision
- 📊 **Task Status**: View current task status and accumulated time
- 📝 **Task History**: List all tasks with durations and status
- 🚀 **Fast & Lightweight**: Built in Rust for performance
- 💻 **Cross-platform**: Works on Linux, macOS, and Windows

## Installation

### From Source

```bash
git clone https://github.com/shubhendud-turing/CLI_Task_Timer.git
cd CLI_Task_Timer
cargo install --path . --bin tt
```

After installation, the `tt` command will be available globally.

## Usage

### Getting Help

```bash
# Show general help
tt --help

# Show help for specific command
tt start --help
```

### Version Information

```bash
tt --version
```

### Starting a Task

Start a new task with a descriptive label:

```bash
# Using long name
 tt start "Working on API implementation"

# Using short name
 tt s "Working on API implementation"
```

**Note**: Starting a new task automatically pauses any currently running task.

### Pausing a Task

Pause the currently running task:

```bash
# Using long name
 tt pause

# Using short name
 tt p
```

This will stop the timer and accumulate the elapsed time. You'll see output like:

```text
Paused task. Current Task: Working on API implementation [⏸️  Paused] - 15m 32s
```

### Resuming a Task

Resume the currently paused task:

```bash
# Using long name
 tt resume

# Using short name
 tt r
```

### Completing a Task

Mark the currently active task as completed:

```bash
# Using long name
 tt complete

# Using short name
 tt c
```

This will:

- Stop the timer if the task is running
- Mark the task as completed
- Clear the active task status
- Preserve the task in history for tracking

Example output:

```text
Completed task: 'Working on API implementation'
```

After completing a task, `tt status` will show `No active task`.

### Checking Status

View the current task status:

```bash
tt status
```

Example output:

- With active task: `Current Task: Working on API implementation [🏃 Running] - 8m 15s`
- No active task: `No active task`

### Listing All Tasks

View all tasks with their durations and status:

```bash
# Using long name
 tt list

# Using short name
 tt l
```

Example output:

```text
Task Summary (3 tasks):
========================================
1. Working on API implementation [⏸️  Paused] - 25m 47s (Created: 2025-10-30 14:30:15 UTC)
2. Code review session [✅ Completed] - 1h 15m 32s (Created: 2025-10-30 13:00:22 UTC)
3. Writing documentation [🏃 Running] - 12m 8s (Created: 2025-10-30 15:45:10 UTC)

========================================
Total Time: 1h 53m 27s
Running: 1 | Paused: 1 | Completed: 1
```

### Renaming Tasks

Rename a task to fix typos or update descriptions:

```bash
# Using long name
 tt rename 2 "Updated task label"
# Using short alias
 tt e 2 "Fixed typo in label"
```

Example output:

```text
Task renamed from "Old Label" to "New Label"
```

**Note**: You can rename any task (running, paused, or completed) without affecting its timing data or status.

Error handling examples:

```bash
# Invalid index (out of bounds)
tt rename 99 "New Label"
Error: Invalid state: Task index 99 is out of bounds. Valid range: 1-3

# Empty or whitespace-only label
tt rename 1 "   "
Error: Invalid state: Task label cannot be empty or whitespace-only

# Empty task list
tt rename 1 "New Label"
Error: No tasks available to rename
```

### Deleting Tasks

Delete a specific task by index:

```bash
# Using long name
 tt delete 2
# Using short alias
 tt d 2
```

Delete all completed tasks:

```bash
# Using long name
 tt delete --completed
# Using short alias
 tt d --completed
```

Example output:

```text
Task "Task 2" deleted successfully
1 completed task(s) deleted successfully
No completed tasks to delete
```

Error handling examples:

```bash
# Invalid index (out of bounds)
tt delete 99
Error: Invalid state: Task index 99 is out of bounds. Valid range: 1-2

# Attempting to delete active task
tt delete 1
Error: Invalid state: Cannot delete task 'Running Task' - task is currently running. Please pause or complete it first.

# Empty task list
tt delete 1
Error: No tasks available to delete

# No completed tasks found
tt delete --completed
No completed tasks to delete
```

## Common Workflows

### Basic Session

```bash
# Start working on a feature
tt start "Implementing user authentication"

# Work for a while...
# Take a break
tt pause

# Resume after break
tt resume

# Check how much time spent
tt status

# Finish the task
tt complete
```

### Multiple Tasks

```bash
# Start first task
tt start "Bug fixing"

# Switch to urgent task (automatically pauses first)
tt start "Urgent client request"

# Complete the urgent task
tt complete

# View all tasks
tt list

# Resume first task to finish it
tt resume

# Complete when done
tt complete
```

## Error Handling

The tool provides clear error messages for invalid operations:

- Trying to pause when no task is running
- Trying to resume a task that's already running
- Trying to complete when no task is active
- Missing required arguments

Example:

```bash
$ tt pause
Error: No active task to operate on

$ tt complete
Error: No active task to operate on
```

## Performance

- **Startup time**: < 10ms
- **Memory usage**: < 5MB
- **Accuracy**: Nanosecond precision timing
- **File size**: < 1MB optimized binary

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup, testing instructions, and contribution guidelines.
