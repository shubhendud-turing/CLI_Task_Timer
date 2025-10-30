use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Represents the current status of a task
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum TaskStatus {
    /// Task is currently running and timing
    Running,
    /// Task has been paused, time accumulation stopped
    Paused,
    /// Task has been completed
    Completed,
}

/// Represents a single task with timing information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Task {
    /// User-provided label for the task
    pub(crate) label: String,
    /// Current status of the task
    pub(crate) status: TaskStatus,
    /// When the task was initially created
    pub(crate) created_at: DateTime<Utc>,
    /// When the task was last started (for current session)
    pub(crate) started_at: Option<DateTime<Utc>>,
    /// Accumulated duration from all previous sessions
    pub(crate) accumulated_duration: Duration,
}

impl Task {
    /// Creates a new task with the given label and starts it immediately
    pub(crate) fn new(label: String) -> Self {
        let now = Utc::now();
        Self {
            label,
            status: TaskStatus::Running,
            created_at: now,
            started_at: Some(now),
            accumulated_duration: Duration::ZERO,
        }
    }

    /// Pauses the task, accumulating the elapsed time since it was started
    pub(crate) fn pause(&mut self) -> Result<(), TaskError> {
        match self.status {
            TaskStatus::Running => {
                if let Some(started_at) = self.started_at {
                    let elapsed = Utc::now()
                        .signed_duration_since(started_at)
                        .to_std()
                        .map_err(|_| TaskError::InvalidDuration)?;

                    self.accumulated_duration += elapsed;
                    self.status = TaskStatus::Paused;
                    self.started_at = None;
                    Ok(())
                } else {
                    Err(TaskError::InvalidState {
                        message: "Task is running but has no start time".to_string(),
                    })
                }
            },
            TaskStatus::Paused => Err(TaskError::TaskAlreadyPaused),
            TaskStatus::Completed => Err(TaskError::TaskCompleted),
        }
    }

    /// Resumes a paused task
    pub(crate) fn resume(&mut self) -> Result<(), TaskError> {
        match self.status {
            TaskStatus::Paused => {
                self.status = TaskStatus::Running;
                self.started_at = Some(Utc::now());
                Ok(())
            },
            TaskStatus::Running => Err(TaskError::TaskAlreadyRunning),
            TaskStatus::Completed => Err(TaskError::TaskCompleted),
        }
    }

    #[allow(dead_code)]
    /// Completes the task, finalizing its total duration
    pub(crate) fn complete(&mut self) -> Result<(), TaskError> {
        match self.status {
            TaskStatus::Running => {
                self.pause()?;
                self.status = TaskStatus::Completed;
                Ok(())
            },
            TaskStatus::Paused => {
                self.status = TaskStatus::Completed;
                Ok(())
            },
            TaskStatus::Completed => Err(TaskError::TaskCompleted),
        }
    }

    /// Gets the total duration of the task, including current session if running
    pub(crate) fn total_duration(&self) -> Duration {
        let mut total = self.accumulated_duration;

        if let (TaskStatus::Running, Some(started_at)) = (&self.status, self.started_at) {
            let current_session = Utc::now()
                .signed_duration_since(started_at)
                .to_std()
                .unwrap_or(Duration::ZERO);
            total += current_session;
        }

        total
    }

    /// Returns true if the task is currently running
    pub(crate) fn is_running(&self) -> bool {
        matches!(self.status, TaskStatus::Running)
    }

    /// Returns true if the task is currently paused
    pub(crate) fn is_paused(&self) -> bool {
        matches!(self.status, TaskStatus::Paused)
    }

    /// Returns true if the task is completed
    pub(crate) fn is_completed(&self) -> bool {
        matches!(self.status, TaskStatus::Completed)
    }
}

/// Manages multiple tasks and enforces business rules
#[derive(Debug, Default)]
pub(crate) struct TaskManager {
    /// List of all tasks
    tasks: Vec<Task>,
    /// Index of the currently active (running or paused) task
    active_task_index: Option<usize>,
}

#[allow(dead_code)]
impl TaskManager {
    /// Creates a new empty task manager
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Starts a new task with the given label
    /// If there's already a running task, it will be paused first
    pub(crate) fn start_task(&mut self, label: String) -> Result<usize, TaskError> {
        // Pause any currently running task
        if let Some(index) = self.active_task_index {
            if self.tasks[index].is_running() {
                self.tasks[index].pause()?;
            }
        }

        // Create and add the new task
        let task = Task::new(label);
        self.tasks.push(task);
        let task_index = self.tasks.len() - 1;
        self.active_task_index = Some(task_index);

        Ok(task_index)
    }

    /// Pauses the currently active task
    pub(crate) fn pause_current_task(&mut self) -> Result<(), TaskError> {
        match self.active_task_index {
            Some(index) => {
                self.tasks[index].pause()?;
                Ok(())
            },
            None => Err(TaskError::NoActiveTask),
        }
    }

    /// Resumes the currently active task (if it's paused)
    pub(crate) fn resume_current_task(&mut self) -> Result<(), TaskError> {
        match self.active_task_index {
            Some(index) => {
                self.tasks[index].resume()?;
                Ok(())
            },
            None => Err(TaskError::NoActiveTask),
        }
    }

    /// Gets a reference to the currently active task
    pub(crate) fn current_task(&self) -> Option<&Task> {
        self.active_task_index.map(|index| &self.tasks[index])
    }

    /// Gets all tasks
    pub(crate) fn all_tasks(&self) -> &[Task] {
        &self.tasks
    }

    /// Gets the number of tasks
    pub(crate) fn task_count(&self) -> usize {
        self.tasks.len()
    }

    /// Checks if there's a currently running task
    pub(crate) fn has_running_task(&self) -> bool {
        self.current_task()
            .map(|task| task.is_running())
            .unwrap_or(false)
    }
}

#[allow(dead_code)]
/// Errors that can occur during task operations
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub(crate) enum TaskError {
    /// No task is currently active
    #[error("No active task to operate on")]
    NoActiveTask,

    /// Task is already running
    #[error("Task is already running")]
    TaskAlreadyRunning,

    /// Task is already paused
    #[error("Task is already paused")]
    TaskAlreadyPaused,

    /// Task has been completed and cannot be modified
    #[error("Task has been completed and cannot be modified")]
    TaskCompleted,

    /// Invalid duration calculation
    #[error("Invalid duration calculation")]
    InvalidDuration,

    /// Invalid state transition
    #[error("Invalid state: {message}")]
    InvalidState { message: String },

    /// Task not found with the given identifier
    #[error("Task not found with id: {id}")]
    TaskNotFound { id: usize },

    /// I/O error occurred during task operations
    #[error("I/O error")]
    IoError(#[source] std::io::Error),

    /// Serialization error
    #[error("Serialization error")]
    SerializationError(#[source] serde_json::Error),

    /// Time-related error
    #[error("Time calculation error: {details}")]
    TimeError { details: String },
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration as StdDuration;

    #[test]
    fn test_new_task_creation() {
        let task = Task::new("Test Task".to_string());

        assert_eq!(task.label, "Test Task");
        assert_eq!(task.status, TaskStatus::Running);
        assert!(task.started_at.is_some());
        assert_eq!(task.accumulated_duration, Duration::ZERO);
        assert!(task.is_running());
        assert!(!task.is_paused());
        assert!(!task.is_completed());
    }

    #[test]
    fn test_task_pause() {
        let mut task = Task::new("Test Task".to_string());

        // Small delay to ensure measurable duration
        thread::sleep(StdDuration::from_millis(10));

        let result = task.pause();
        assert!(result.is_ok());
        assert_eq!(task.status, TaskStatus::Paused);
        assert!(task.started_at.is_none());
        assert!(task.accumulated_duration > Duration::ZERO);
        assert!(task.is_paused());
        assert!(!task.is_running());
    }

    #[test]
    fn test_task_pause_already_paused() {
        let mut task = Task::new("Test Task".to_string());
        task.pause().unwrap();

        let result = task.pause();
        assert!(result.is_err());
        match result.unwrap_err() {
            TaskError::TaskAlreadyPaused => {},
            _ => panic!("Expected TaskAlreadyPaused error"),
        }
    }

    #[test]
    fn test_task_resume() {
        let mut task = Task::new("Test Task".to_string());
        task.pause().unwrap();

        let result = task.resume();
        assert!(result.is_ok());
        assert_eq!(task.status, TaskStatus::Running);
        assert!(task.started_at.is_some());
        assert!(task.is_running());
        assert!(!task.is_paused());
    }

    #[test]
    fn test_task_resume_already_running() {
        let mut task = Task::new("Test Task".to_string());

        let result = task.resume();
        assert!(result.is_err());
        match result.unwrap_err() {
            TaskError::TaskAlreadyRunning => {},
            _ => panic!("Expected TaskAlreadyRunning error"),
        }
    }

    #[test]
    fn test_task_total_duration() {
        let mut task = Task::new("Test Task".to_string());

        // Run for a bit
        thread::sleep(StdDuration::from_millis(10));
        task.pause().unwrap();

        let duration_after_pause = task.total_duration();
        assert!(duration_after_pause > Duration::ZERO);

        // Resume and run again
        task.resume().unwrap();
        thread::sleep(StdDuration::from_millis(10));

        let duration_while_running = task.total_duration();
        assert!(duration_while_running > duration_after_pause);
    }

    #[test]
    fn test_task_manager_start_task() {
        let mut manager = TaskManager::new();

        let task_index = manager.start_task("First Task".to_string()).unwrap();
        assert_eq!(task_index, 0);
        assert_eq!(manager.task_count(), 1);
        assert!(manager.has_running_task());

        let current_task = manager.current_task().unwrap();
        assert_eq!(current_task.label, "First Task");
        assert!(current_task.is_running());
    }

    #[test]
    fn test_task_manager_start_multiple_tasks() {
        let mut manager = TaskManager::new();

        // Start first task
        manager.start_task("First Task".to_string()).unwrap();
        thread::sleep(StdDuration::from_millis(10));

        // Start second task (should pause the first)
        manager.start_task("Second Task".to_string()).unwrap();

        assert_eq!(manager.task_count(), 2);

        // First task should be paused
        assert!(manager.all_tasks()[0].is_paused());

        // Second task should be running
        let current_task = manager.current_task().unwrap();
        assert_eq!(current_task.label, "Second Task");
        assert!(current_task.is_running());
    }

    #[test]
    fn test_task_manager_pause_current() {
        let mut manager = TaskManager::new();
        manager.start_task("Test Task".to_string()).unwrap();

        thread::sleep(StdDuration::from_millis(10));

        let result = manager.pause_current_task();
        assert!(result.is_ok());

        let current_task = manager.current_task().unwrap();
        assert!(current_task.is_paused());
        assert!(!manager.has_running_task());
    }

    #[test]
    fn test_task_manager_pause_no_active_task() {
        let mut manager = TaskManager::new();

        let result = manager.pause_current_task();
        assert!(result.is_err());
        match result.unwrap_err() {
            TaskError::NoActiveTask => {},
            _ => panic!("Expected NoActiveTask error"),
        }
    }

    #[test]
    fn test_task_manager_resume_current() {
        let mut manager = TaskManager::new();
        manager.start_task("Test Task".to_string()).unwrap();
        manager.pause_current_task().unwrap();

        let result = manager.resume_current_task();
        assert!(result.is_ok());

        let current_task = manager.current_task().unwrap();
        assert!(current_task.is_running());
        assert!(manager.has_running_task());
    }

    #[test]
    fn test_task_complete() {
        let mut task = Task::new("Test Task".to_string());
        thread::sleep(StdDuration::from_millis(10));

        let result = task.complete();
        assert!(result.is_ok());
        assert!(task.is_completed());
        assert!(task.total_duration() > Duration::ZERO);

        // Should not be able to pause/resume completed task
        let pause_result = task.pause();
        assert!(pause_result.is_err());
        match pause_result.unwrap_err() {
            TaskError::TaskCompleted => {},
            _ => panic!("Expected TaskCompleted error"),
        }

        let resume_result = task.resume();
        assert!(resume_result.is_err());
        match resume_result.unwrap_err() {
            TaskError::TaskCompleted => {},
            _ => panic!("Expected TaskCompleted error"),
        }
    }
}
