use crate::state::AppState;
use crate::task::TaskDto;
use tauri::State;

/// Retrieves all tasks as DTOs for the frontend.
#[tauri::command]
pub fn get_tasks(state: State<AppState>) -> Result<Vec<TaskDto>, String> {
    let tasks = state.tasks.lock()
        .map_err(|_| "Failed to acquire lock".to_string())?;

    let mut task_list: Vec<TaskDto> = tasks
        .values()
        .map(TaskDto::from)
        .collect();

    // Sort by creation date
    task_list.sort_by(|a, b| a.created_at.cmp(&b.created_at));

    Ok(task_list)
}

/// Adds a new task with the given name.
#[tauri::command]
pub fn add_task(state: State<AppState>, name: String) -> Result<(), String> {
    let trimmed_name = name.trim().to_string();
    
    if trimmed_name.is_empty() {
        return Err("Task name cannot be empty".to_string());
    }

    state.add_task(trimmed_name)
}

/// Starts time tracking for the specified task.
#[tauri::command]
pub fn start_task(state: State<AppState>, name: String) -> Result<(), String> {
    state.start_task(&name)
}

/// Stops time tracking for the specified task.
#[tauri::command]
pub fn stop_task(state: State<AppState>, name: String) -> Result<(), String> {
    state.stop_task(&name)
}

/// Resets the time for the specified task.
#[tauri::command]
pub fn reset_task(state: State<AppState>, name: String) -> Result<(), String> {
    state.reset_task(&name)
}

/// Deletes the specified task.
#[tauri::command]
pub fn delete_task(state: State<AppState>, name: String) -> Result<(), String> {
    state.delete_task(&name)
}

/// Exports all tasks to a text file at the specified path.
#[tauri::command]
pub fn export_tasks(state: State<AppState>, path: String) -> Result<(), String> {
    state.export_tasks(&path)
}