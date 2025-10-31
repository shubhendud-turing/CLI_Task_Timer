use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;

const MAX_TASKS: usize = 10;

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
#[derive(Debug, Default, Serialize, Deserialize)]
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
        if let Some(index) = self.active_task_index
            && self.tasks[index].is_running()
        {
            self.tasks[index].pause()?;
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

    /// Completes the currently active task and clears the active task status
    pub(crate) fn complete_current_task(&mut self) -> Result<(), TaskError> {
        match self.active_task_index {
            Some(index) => {
                self.tasks[index].complete()?;
                self.active_task_index = None;
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

    /// Load existing TaskManager from file or create new one
    pub(crate) fn load_or_create() -> Result<Self, TaskError> {
        match Self::load_from_file() {
            Ok(mut manager) => {
                manager.cleanup_old_tasks();
                Ok(manager)
            },
            Err(_) => Ok(Self::new()),
        }
    }

    /// Load TaskManager from the JSON file
    fn load_from_file() -> Result<Self, TaskError> {
        let path = Self::get_config_path()?;
        let content = fs::read_to_string(path)?;
        let manager: TaskManager = serde_json::from_str(&content)?;
        Ok(manager)
    }

    /// Save current TaskManager state to JSON file
    pub(crate) fn save(&self) -> Result<(), TaskError> {
        let path = Self::get_config_path()?;

        // Ensure the parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Serialize to JSON
        let json = serde_json::to_string_pretty(self)?;

        // Write to temporary file first for atomicity
        let temp_path = path.with_extension("tmp");
        let mut file = fs::File::create(&temp_path)?;
        file.write_all(json.as_bytes())?;
        file.sync_all()?;

        // Atomic rename
        fs::rename(temp_path, path)?;
        Ok(())
    }

    /// Get the cross-platform config file path
    fn get_config_path() -> Result<PathBuf, TaskError> {
        // Check for test override first
        if let Ok(test_dir) = std::env::var("TT_CONFIG_DIR") {
            return Ok(PathBuf::from(test_dir).join("tasks.json"));
        }

        let config_dir = dirs::config_dir().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not find config directory",
            )
        })?;

        let tt_dir = config_dir.join("tt");
        Ok(tt_dir.join("tasks.json"))
    }

    /// Remove oldest completed tasks if we have more than 10 total tasks
    fn cleanup_old_tasks(&mut self) {
        if self.tasks.len() <= MAX_TASKS {
            return;
        }

        // Separate active and completed tasks
        let active_task_id = self.active_task_index;
        let mut active_tasks = Vec::new();
        let mut completed_tasks = Vec::new();

        for (index, task) in self.tasks.iter().enumerate() {
            if Some(index) == active_task_id || !task.is_completed() {
                active_tasks.push((index, task.clone()));
            } else {
                completed_tasks.push((index, task.clone()));
            }
        }

        // Sort completed tasks by creation time (oldest first)
        completed_tasks.sort_by(|a, b| a.1.created_at.cmp(&b.1.created_at));

        // Keep active tasks + newest completed tasks up to MAX_TASKS
        let mut new_tasks = Vec::new();
        let mut new_active_index = None;

        // Add active tasks first
        for (old_index, task) in active_tasks {
            if Some(old_index) == active_task_id {
                new_active_index = Some(new_tasks.len());
            }
            new_tasks.push(task);
        }

        // Add newest completed tasks
        let remaining_slots = MAX_TASKS.saturating_sub(new_tasks.len());
        let keep_completed = completed_tasks.len().saturating_sub(remaining_slots);

        for (_, task) in completed_tasks.into_iter().skip(keep_completed) {
            new_tasks.push(task);
        }

        self.tasks = new_tasks;
        self.active_task_index = new_active_index;
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
    IoError(#[from] std::io::Error),

    /// Serialization error
    #[error("Serialization error")]
    SerializationError(#[from] serde_json::Error),

    /// Time-related error
    #[error("Time calculation error: {details}")]
    TimeError { details: String },
}

#[cfg(test)]
mod task_tests;
