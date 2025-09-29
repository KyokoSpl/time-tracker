use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use chrono::{DateTime, Local};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    #[serde(with = "duration_serde")]
    pub total_time: Duration,
    #[serde(skip)] // Don't serialize Instant as it's not meaningful across sessions
    pub start_time: Option<Instant>,
    #[serde(skip)] // Always start as not running when loading
    pub is_running: bool,
    pub created_at: DateTime<Local>,
}

impl Task {
    pub fn new(name: String) -> Self {
        Task {
            name,
            total_time: Duration::new(0, 0),
            start_time: None,
            is_running: false,
            created_at: Local::now(),
        }
    }
    
    pub fn start(&mut self) {
        if !self.is_running {
            self.start_time = Some(Instant::now());
            self.is_running = true;
        }
    }
    
    pub fn stop(&mut self) {
        if self.is_running {
            if let Some(start) = self.start_time {
                self.total_time += start.elapsed();
            }
            self.is_running = false;
            self.start_time = None;
        }
    }
    
    pub fn reset(&mut self) {
        self.stop();
        self.total_time = Duration::new(0, 0);
    }
    
    pub fn get_current_time(&self) -> Duration {
        let mut current_time = self.total_time;
        if self.is_running {
            if let Some(start) = self.start_time {
                current_time += start.elapsed();
            }
        }
        current_time
    }
    
    pub fn format_duration(duration: Duration) -> String {
        let total_seconds = duration.as_secs();
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
}

// Custom serialization for Duration
mod duration_serde {
    use serde::{Deserialize, Serialize, Deserializer, Serializer};
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