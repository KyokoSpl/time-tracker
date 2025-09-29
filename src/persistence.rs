use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use crate::task::Task;

pub struct Persistence;

impl Persistence {
    pub fn get_save_path() -> PathBuf {
        // Get the user's config directory or fallback to current directory
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
        
        config_dir.join("time_tracker_data.json")
    }

    pub fn save_tasks(tasks: &HashMap<String, Task>) {
        let save_path = Self::get_save_path();
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = save_path.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                eprintln!("Failed to create config directory: {}", e);
                return;
            }
        }

        match serde_json::to_string_pretty(tasks) {
            Ok(json_data) => {
                if let Err(e) = fs::write(&save_path, json_data) {
                    eprintln!("Failed to save tasks: {}", e);
                } else {
                    println!("Tasks saved to: {}", save_path.display());
                }
            }
            Err(e) => {
                eprintln!("Failed to serialize tasks: {}", e);
            }
        }
    }

    pub fn load_tasks() -> HashMap<String, Task> {
        let save_path = Self::get_save_path();
        
        if !save_path.exists() {
            println!("No saved tasks found, starting fresh");
            return HashMap::new();
        }

        match fs::read_to_string(&save_path) {
            Ok(json_data) => {
                match serde_json::from_str::<HashMap<String, Task>>(&json_data) {
                    Ok(loaded_tasks) => {
                        println!("Loaded {} tasks from: {}", loaded_tasks.len(), save_path.display());
                        loaded_tasks
                    }
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

    pub fn export_to_txt(tasks: &HashMap<String, Task>) {
        use chrono::Local;
        use std::fs::File;
        use std::io::Write;
        
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Text files", &["txt"])
            .set_file_name("time_tracker_export.txt")
            .save_file()
        {
            match File::create(&path) {
                Ok(mut file) => {
                    writeln!(file, "Time Tracker Export").unwrap();
                    writeln!(file, "Generated on: {}", Local::now().format("%Y-%m-%d %H:%M:%S")).unwrap();
                    writeln!(file, "").unwrap();
                    
                    let mut sorted_tasks: Vec<_> = tasks.iter().collect();
                    sorted_tasks.sort_by(|a, b| a.1.created_at.cmp(&b.1.created_at));
                    
                    for (name, task) in sorted_tasks {
                        let total_time = Task::format_duration(task.get_current_time());
                        let status = if task.is_running { " (Running)" } else { "" };
                        
                        writeln!(file, "Task: {}", name).unwrap();
                        writeln!(file, "Total Time: {}{}", total_time, status).unwrap();
                        writeln!(file, "Created: {}", task.created_at.format("%Y-%m-%d %H:%M:%S")).unwrap();
                        writeln!(file, "").unwrap();
                    }
                    
                    println!("Export successful: {}", path.display());
                }
                Err(e) => {
                    eprintln!("Failed to export: {}", e);
                }
            }
        }
    }
}