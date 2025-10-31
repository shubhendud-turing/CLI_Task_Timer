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

    #[test]
    fn test_serialize_deserialize_task_manager() {
        let mut manager = TaskManager::new();
        let _task_id = manager.start_task("Test Task".to_string()).unwrap();
        manager.pause_current_task().unwrap();

        // Serialize to JSON
        let json = serde_json::to_string(&manager).unwrap();
        assert!(json.contains("Test Task"));
        assert!(json.contains("Paused"));

        // Deserialize from JSON
        let deserialized: TaskManager = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.tasks.len(), 1);
        assert_eq!(deserialized.active_task_index, Some(0));
        assert_eq!(deserialized.tasks[0].label, "Test Task");
        assert!(deserialized.tasks[0].is_paused());
    }

    #[test]
    fn test_cleanup_old_tasks() {
        let mut manager = TaskManager::new();

        // Create 15 tasks (more than the 10 limit)
        for i in 0..15 {
            let _task_id = manager.start_task(format!("Task {}", i)).unwrap();
            // Complete old tasks by directly modifying tasks (simulating completed state)
            if i < 10 {
                // Access task directly for testing purposes
                if let Some(index) = manager.active_task_index {
                    manager.tasks[index].status = TaskStatus::Completed;
                    manager.active_task_index = None;
                }
            }
        }

        assert_eq!(manager.tasks.len(), 15);

        // Run cleanup
        manager.cleanup_old_tasks();

        // Should have at most 10 tasks
        assert!(manager.tasks.len() <= 10);

        // Should preserve the most recent active/incomplete tasks
        let has_recent_tasks = manager
            .tasks
            .iter()
            .any(|task| task.label.contains("Task 14") || task.label.contains("Task 13"));
        assert!(has_recent_tasks);
    }

    #[test]
    fn test_cleanup_preserves_active_task() {
        let mut manager = TaskManager::new();

        // Create many completed tasks
        for i in 0..12 {
            let _task_id = manager.start_task(format!("Completed Task {}", i)).unwrap();
            // Simulate completion by setting status directly
            if let Some(index) = manager.active_task_index {
                manager.tasks[index].status = TaskStatus::Completed;
                manager.active_task_index = None;
            }
        }

        // Create one active task
        let _active_id = manager
            .start_task("Important Active Task".to_string())
            .unwrap();

        assert_eq!(manager.tasks.len(), 13);

        // Run cleanup
        manager.cleanup_old_tasks();

        // Should still have the active task
        assert!(manager.active_task_index.is_some());
        let current_task = manager.current_task().unwrap();
        assert_eq!(current_task.label, "Important Active Task");
        assert!(manager.tasks.len() <= 10);
    }

    #[test]
    fn test_get_config_path() {
        let path_result = TaskManager::get_config_path();
        assert!(path_result.is_ok());

        let path = path_result.unwrap();
        assert!(path.to_string_lossy().contains("tt"));
        assert!(path.to_string_lossy().ends_with("tasks.json"));
    }

    #[test]
    fn test_complete_current_task_running() {
        let mut manager = TaskManager::new();
        manager.start_task("Test Task".to_string()).unwrap();
        thread::sleep(StdDuration::from_millis(10));

        let result = manager.complete_current_task();
        assert!(result.is_ok());

        // Should have no active task after completion
        assert!(manager.current_task().is_none());
        assert_eq!(manager.active_task_index, None);

        // Task should be completed
        assert_eq!(manager.tasks.len(), 1);
        assert!(manager.tasks[0].is_completed());
        assert!(manager.tasks[0].total_duration() > Duration::ZERO);
    }

    #[test]
    fn test_complete_current_task_paused() {
        let mut manager = TaskManager::new();
        manager.start_task("Test Task".to_string()).unwrap();
        thread::sleep(StdDuration::from_millis(10));
        manager.pause_current_task().unwrap();

        let result = manager.complete_current_task();
        assert!(result.is_ok());

        // Should have no active task after completion
        assert!(manager.current_task().is_none());
        assert_eq!(manager.active_task_index, None);

        // Task should be completed
        assert_eq!(manager.tasks.len(), 1);
        assert!(manager.tasks[0].is_completed());
    }

    #[test]
    fn test_complete_current_task_no_active() {
        let mut manager = TaskManager::new();

        let result = manager.complete_current_task();
        assert!(result.is_err());
        match result.unwrap_err() {
            TaskError::NoActiveTask => {},
            _ => panic!("Expected NoActiveTask error"),
        }
    }

    #[test]
    fn test_complete_task_cannot_be_resumed() {
        let mut manager = TaskManager::new();
        manager.start_task("Test Task".to_string()).unwrap();
        manager.complete_current_task().unwrap();

        // Task is completed and no longer active
        // Trying to resume should fail since there's no active task
        let result = manager.resume_current_task();
        assert!(result.is_err());
        match result.unwrap_err() {
            TaskError::NoActiveTask => {},
            _ => panic!("Expected NoActiveTask error"),
        }
    }

    #[test]
    fn test_multiple_tasks_with_completion() {
        let mut manager = TaskManager::new();

        // Start and complete first task
        manager.start_task("Task 1".to_string()).unwrap();
        thread::sleep(StdDuration::from_millis(10));
        manager.complete_current_task().unwrap();

        // Start and complete second task
        manager.start_task("Task 2".to_string()).unwrap();
        thread::sleep(StdDuration::from_millis(10));
        manager.complete_current_task().unwrap();

        // Start third task but don't complete it
        manager.start_task("Task 3".to_string()).unwrap();

        // Should have 3 tasks total
        assert_eq!(manager.tasks.len(), 3);

        // First two should be completed
        assert!(manager.tasks[0].is_completed());
        assert!(manager.tasks[1].is_completed());

        // Third should be running
        assert!(manager.tasks[2].is_running());
        assert!(manager.current_task().is_some());
        assert_eq!(manager.current_task().unwrap().label, "Task 3");
    }

    // Note: File I/O tests would be more complex and require temporary directories
    // They're better suited for integration tests to avoid filesystem side effects
}
