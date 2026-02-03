use crate::persistence::Persistence;
use crate::task::Task;
use std::collections::HashMap;
use std::sync::Mutex;

/// Thread-safe application state holding all tasks.
pub struct AppState {
    pub tasks: Mutex<HashMap<String, Task>>,
}

impl AppState {
    /// Creates a new AppState, loading any persisted tasks.
    pub fn new() -> Self {
        let tasks = Persistence::load_tasks();
        Self {
            tasks: Mutex::new(tasks),
        }
    }

    /// Saves the current state to persistent storage.
    /// 
    /// # Arguments
    /// * `tasks` - Reference to the tasks HashMap (caller should hold the lock)
    /// 
    /// # Returns
    /// Result indicating success or error message
    pub fn save(&self, tasks: &HashMap<String, Task>) -> Result<(), String> {
        Persistence::save_tasks(tasks)
    }

    /// Adds a new task with the given name.
    /// 
    /// # Arguments
    /// * `name` - Name of the task to add
    /// 
    /// # Returns
    /// Result with unit on success, or error if task already exists
    pub fn add_task(&self, name: String) -> Result<(), String> {
        let mut tasks = self.tasks.lock()
            .map_err(|_| "Failed to acquire lock")?;

        if tasks.contains_key(&name) {
            return Err(format!("Task '{}' already exists", name));
        }

        tasks.insert(name.clone(), Task::new(name));
        self.save(&tasks)?;
        Ok(())
    }

    /// Starts time tracking for the specified task.
    /// 
    /// # Arguments
    /// * `name` - Name of the task to start
    /// 
    /// # Returns
    /// Result with unit on success, or error if task not found
    pub fn start_task(&self, name: &str) -> Result<(), String> {
        let mut tasks = self.tasks.lock()
            .map_err(|_| "Failed to acquire lock")?;

        let task = tasks.get_mut(name)
            .ok_or_else(|| format!("Task '{}' not found", name))?;

        task.start();
        self.save(&tasks)?;
        Ok(())
    }

    /// Stops time tracking for the specified task.
    /// 
    /// # Arguments
    /// * `name` - Name of the task to stop
    /// 
    /// # Returns
    /// Result with unit on success, or error if task not found
    pub fn stop_task(&self, name: &str) -> Result<(), String> {
        let mut tasks = self.tasks.lock()
            .map_err(|_| "Failed to acquire lock")?;

        let task = tasks.get_mut(name)
            .ok_or_else(|| format!("Task '{}' not found", name))?;

        task.stop();
        self.save(&tasks)?;
        Ok(())
    }

    /// Resets the time for the specified task.
    /// 
    /// # Arguments
    /// * `name` - Name of the task to reset
    /// 
    /// # Returns
    /// Result with unit on success, or error if task not found
    pub fn reset_task(&self, name: &str) -> Result<(), String> {
        let mut tasks = self.tasks.lock()
            .map_err(|_| "Failed to acquire lock")?;

        let task = tasks.get_mut(name)
            .ok_or_else(|| format!("Task '{}' not found", name))?;

        task.reset();
        self.save(&tasks)?;
        Ok(())
    }

    /// Deletes the specified task.
    /// 
    /// # Arguments
    /// * `name` - Name of the task to delete
    /// 
    /// # Returns
    /// Result with unit on success, or error if task not found
    pub fn delete_task(&self, name: &str) -> Result<(), String> {
        let mut tasks = self.tasks.lock()
            .map_err(|_| "Failed to acquire lock")?;

        if tasks.remove(name).is_none() {
            return Err(format!("Task '{}' not found", name));
        }

        self.save(&tasks)?;
        Ok(())
    }

    /// Exports all tasks to a text file.
    /// 
    /// # Arguments
    /// * `path` - Path where the export file should be written
    /// 
    /// # Returns
    /// Result with unit on success, or error message on failure
    pub fn export_tasks(&self, path: &str) -> Result<(), String> {
        let tasks = self.tasks.lock()
            .map_err(|_| "Failed to acquire lock")?;

        Persistence::export_to_txt(&tasks, path)
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_get_task() {
        let state = AppState::new();
        
        // Clear any existing test tasks
        {
            let mut tasks = state.tasks.lock().unwrap();
            tasks.clear();
        }

        let result = state.add_task("Test Task".to_string());
        assert!(result.is_ok());

        let tasks = state.tasks.lock().unwrap();
        assert!(tasks.contains_key("Test Task"));
    }

    #[test]
    fn test_duplicate_task_error() {
        let state = AppState::new();
        
        // Clear and add a task
        {
            let mut tasks = state.tasks.lock().unwrap();
            tasks.clear();
            tasks.insert("Duplicate".to_string(), Task::new("Duplicate".to_string()));
        }

        let result = state.add_task("Duplicate".to_string());
        assert!(result.is_err());
    }
}