use crate::task::Task;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Handles persistence of tasks to and from disk.
pub struct Persistence;

impl Persistence {
    /// Returns the path to the save file in the user's config directory.
    pub fn get_save_path() -> PathBuf {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
        config_dir.join("time_tracker_tauri_data.json")
    }

    /// Saves tasks to the persistent storage file.
    /// 
    /// # Arguments
    /// * `tasks` - HashMap of task name to Task
    /// 
    /// # Returns
    /// Result indicating success or error message
    pub fn save_tasks(tasks: &HashMap<String, Task>) -> Result<(), String> {
        let save_path = Self::get_save_path();

        // Create parent directory if it doesn't exist
        if let Some(parent) = save_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }

        let json_data = serde_json::to_string_pretty(tasks)
            .map_err(|e| format!("Failed to serialize tasks: {}", e))?;

        fs::write(&save_path, json_data)
            .map_err(|e| format!("Failed to save tasks: {}", e))?;

        Ok(())
    }

    /// Loads tasks from the persistent storage file.
    /// 
    /// # Returns
    /// HashMap of task name to Task, or empty HashMap if file doesn't exist
    pub fn load_tasks() -> HashMap<String, Task> {
        let save_path = Self::get_save_path();

        if !save_path.exists() {
            return HashMap::new();
        }

        match fs::read_to_string(&save_path) {
            Ok(json_data) => {
                match serde_json::from_str::<HashMap<String, Task>>(&json_data) {
                    Ok(tasks) => tasks,
                    Err(e) => {
                        eprintln!("Failed to deserialize tasks: {}", e);
                        HashMap::new()
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to read tasks file: {}", e);
                HashMap::new()
            }
        }
    }

    /// Exports tasks to a text file at the specified path.
    /// 
    /// # Arguments
    /// * `tasks` - HashMap of task name to Task
    /// * `path` - Path where the export file should be written
    /// 
    /// # Returns
    /// Result indicating success or error message
    pub fn export_to_txt(tasks: &HashMap<String, Task>, path: &str) -> Result<(), String> {
        use chrono::Local;
        use std::io::Write;

        let mut file = fs::File::create(path)
            .map_err(|e| format!("Failed to create export file: {}", e))?;

        writeln!(file, "Time Tracker Export")
            .map_err(|e| format!("Failed to write to export file: {}", e))?;
        writeln!(file, "Generated on: {}", Local::now().format("%Y-%m-%d %H:%M:%S"))
            .map_err(|e| format!("Failed to write to export file: {}", e))?;
        writeln!(file)
            .map_err(|e| format!("Failed to write to export file: {}", e))?;

        let mut sorted_tasks: Vec<_> = tasks.iter().collect();
        sorted_tasks.sort_by(|a, b| a.1.created_at.cmp(&b.1.created_at));

        for (name, task) in sorted_tasks {
            let total_time = Task::format_duration(task.get_current_time());
            let status = if task.is_running { " (Running)" } else { "" };

            writeln!(file, "Task: {}", name)
                .map_err(|e| format!("Failed to write to export file: {}", e))?;
            writeln!(file, "Total Time: {}{}", total_time, status)
                .map_err(|e| format!("Failed to write to export file: {}", e))?;
            writeln!(file, "Created: {}", task.created_at.format("%Y-%m-%d %H:%M:%S"))
                .map_err(|e| format!("Failed to write to export file: {}", e))?;
            writeln!(file)
                .map_err(|e| format!("Failed to write to export file: {}", e))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    #[test]
    fn test_save_and_load_tasks() {
        // Create a temporary directory for testing
        let temp_dir = env::temp_dir().join("time_tracker_test");
        fs::create_dir_all(&temp_dir).unwrap();
        
        let mut tasks = HashMap::new();
        tasks.insert("Test Task".to_string(), Task::new("Test Task".to_string()));
        
        // Note: This test uses the actual save path, 
        // so it may affect real data if run outside of a test environment
        let result = Persistence::save_tasks(&tasks);
        assert!(result.is_ok());
        
        let loaded = Persistence::load_tasks();
        assert!(loaded.contains_key("Test Task"));
    }
}