use serde::{Deserialize, Serialize};
use std::time::Duration;
use chrono::{DateTime, Local};

/// Represents a time-tracked task with accumulated time and state.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    #[serde(with = "duration_serde")]
    pub total_time: Duration,
    #[serde(skip)]
    pub start_timestamp: Option<i64>,
    #[serde(skip)]
    pub is_running: bool,
    pub created_at: DateTime<Local>,
}

impl Task {
    /// Creates a new task with the given name.
    pub fn new(name: String) -> Self {
        Self {
            name,
            total_time: Duration::ZERO,
            start_timestamp: None,
            is_running: false,
            created_at: Local::now(),
        }
    }

    /// Starts time tracking for this task.
    pub fn start(&mut self) {
        if !self.is_running {
            self.start_timestamp = Some(Local::now().timestamp_millis());
            self.is_running = true;
        }
    }

    /// Stops time tracking and accumulates elapsed time.
    pub fn stop(&mut self) {
        if self.is_running {
            if let Some(start) = self.start_timestamp {
                let now = Local::now().timestamp_millis();
                let elapsed_millis = (now - start).max(0) as u64;
                self.total_time += Duration::from_millis(elapsed_millis);
            }
            self.is_running = false;
            self.start_timestamp = None;
        }
    }

    /// Resets the task's accumulated time to zero.
    pub fn reset(&mut self) {
        self.stop();
        self.total_time = Duration::ZERO;
    }

    /// Returns the current total time including any running session.
    pub fn get_current_time(&self) -> Duration {
        let mut current_time = self.total_time;
        if self.is_running {
            if let Some(start) = self.start_timestamp {
                let now = Local::now().timestamp_millis();
                let elapsed_millis = (now - start).max(0) as u64;
                current_time += Duration::from_millis(elapsed_millis);
            }
        }
        current_time
    }

    /// Formats a duration as HH:MM:SS string.
    pub fn format_duration(duration: Duration) -> String {
        let total_seconds = duration.as_secs();
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
}

/// DTO for sending task data to the frontend.
#[derive(Clone, Debug, Serialize)]
pub struct TaskDto {
    pub name: String,
    pub total_time_secs: u64,
    pub formatted_time: String,
    pub is_running: bool,
    pub created_at: String,
}

impl From<&Task> for TaskDto {
    fn from(task: &Task) -> Self {
        let current_time = task.get_current_time();
        Self {
            name: task.name.clone(),
            total_time_secs: current_time.as_secs(),
            formatted_time: Task::format_duration(current_time),
            is_running: task.is_running,
            created_at: task.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}

/// Custom serialization for Duration (stored as seconds).
mod duration_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::time::Duration;

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        duration.as_secs().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = u64::deserialize(deserializer)?;
        Ok(Duration::from_secs(secs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_creation() {
        let task = Task::new("Test Task".to_string());
        assert_eq!(task.name, "Test Task");
        assert_eq!(task.total_time, Duration::ZERO);
        assert!(!task.is_running);
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(Task::format_duration(Duration::from_secs(0)), "00:00:00");
        assert_eq!(Task::format_duration(Duration::from_secs(59)), "00:00:59");
        assert_eq!(Task::format_duration(Duration::from_secs(3661)), "01:01:01");
    }
}