use eframe::egui::{self, Vec2};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use crate::task::Task;
use crate::persistence::Persistence;
use crate::ui::MaterialUI;

#[derive(Default)]
pub struct TimeTrackerApp {
    pub tasks: HashMap<String, Task>,
    pub new_task_name: String,
    pub show_reset_dialog: bool,
    pub task_to_reset: String,
    pub show_delete_dialog: bool,
    pub task_to_delete: String,
    pub dark_mode: bool,
    pub last_save_time: Option<Instant>,
}

impl TimeTrackerApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Configure egui style for Material Design look
        MaterialUI::setup_style(&_cc.egui_ctx);
        
        // Load existing tasks from file
        let mut app = Self::default();
        app.tasks = Persistence::load_tasks();
        app
    }

    fn save_tasks(&self) {
        Persistence::save_tasks(&self.tasks);
    }

    fn export_to_txt(&self) {
        Persistence::export_to_txt(&self.tasks);
    }

    fn toggle_theme(&mut self, ctx: &egui::Context) {
        MaterialUI::toggle_theme(ctx, &mut self.dark_mode);
    }

    fn render_task_card(&mut self, ui: &mut egui::Ui, task_name: &str, task: &Task) {
        let task_name_owned = task_name.to_string();
        let (delete_clicked, reset_clicked, start_stop_clicked) = MaterialUI::material_card(ui, true, |ui| {
            ui.horizontal(|ui| {
                // Task name
                ui.label(egui::RichText::new(task_name).size(16.0).strong());
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Delete button
                    let delete_clicked = MaterialUI::material_button(ui, "🗑", false).clicked();
                    
                    ui.add_space(8.0);
                    
                    // Reset button
                    let reset_clicked = MaterialUI::material_button(ui, "Reset", false).clicked();
                    
                    ui.add_space(8.0);
                    
                    // Start/Stop button
                    let button_text = if task.is_running { "Stop" } else { "Start" };
                    let start_stop_clicked = MaterialUI::material_button(ui, button_text, task.is_running).clicked();
                    
                    ui.add_space(8.0);
                    
                    // Running indicator
                    if task.is_running {
                        ui.spinner();
                        ui.add_space(8.0);
                    }
                    
                    // Time display
                    let time_str = Task::format_duration(task.get_current_time());
                    ui.label(egui::RichText::new(&time_str).size(18.0).monospace());
                    
                    (delete_clicked, reset_clicked, start_stop_clicked)
                }).inner
            }).inner
        });
        
        if delete_clicked {
            self.show_delete_dialog = true;
            self.task_to_delete = task_name_owned.clone();
        }
        
        if reset_clicked {
            self.show_reset_dialog = true;
            self.task_to_reset = task_name_owned;
        }
        
        if start_stop_clicked {
            if let Some(task) = self.tasks.get_mut(task_name) {
                if task.is_running {
                    task.stop();
                } else {
                    task.start();
                }
                // Auto-save when task state changes
                self.save_tasks();
            }
        }
    }
}

impl eframe::App for TimeTrackerApp {
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // Save tasks when the application is closing
        self.save_tasks();
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Request repaint for smooth time updates
        ctx.request_repaint_after(Duration::from_millis(100));
        
        // Periodic save every 30 seconds to capture running task times
        let now = Instant::now();
        let should_save = match self.last_save_time {
            Some(last_save) => now.duration_since(last_save) >= Duration::from_secs(30),
            None => true,
        };
        
        if should_save && self.tasks.values().any(|task| task.is_running) {
            self.save_tasks();
            self.last_save_time = Some(now);
        }
        
        // Top app bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("🕐 Time Tracker");
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Export button
                    if MaterialUI::material_button(ui, "Export", false).clicked() {
                        self.export_to_txt();
                    }
                    
                    ui.add_space(8.0);
                    
                    // Theme toggle button
                    let theme_icon = if self.dark_mode { "🌙" } else { "☀️" };
                    if ui.button(theme_icon).clicked() {
                        self.toggle_theme(ctx);
                    }
                });
            });
        });
        
        // Main content area
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing.y = 12.0;
            
            // Add task section
            let add_clicked = MaterialUI::material_card(ui, false, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Add New Task:").size(14.0));
                    
                    let response = ui.add(
                        egui::TextEdit::singleline(&mut self.new_task_name)
                            .hint_text("Enter task name")
                            .desired_width(200.0)
                    );
                    
                    let add_clicked = MaterialUI::material_button(ui, "Add Task", true).clicked();
                    
                    // Return both response and add_clicked for handling outside
                    (response, add_clicked)
                }).inner
            });
            
            // Handle Enter key or Add button click
            if (add_clicked.0.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) || add_clicked.1 {
                let task_name = self.new_task_name.trim().to_string();
                if !task_name.is_empty() && !self.tasks.contains_key(&task_name) {
                    self.tasks.insert(task_name, Task::new(self.new_task_name.clone()));
                    self.new_task_name.clear();
                    // Auto-save when new task is added
                    self.save_tasks();
                }
            }
            
            ui.separator();
            
            // Tasks list
            if self.tasks.is_empty() {
                ui.vertical_centered(|ui| {
                    ui.add_space(50.0);
                    ui.label(egui::RichText::new("No tasks yet. Add a task to get started!").size(16.0));
                });
            } else {
                // Collect task data first to avoid borrowing issues
                let mut tasks_to_render: Vec<(String, Task)> = self.tasks.iter()
                    .map(|(name, task)| (name.clone(), task.clone()))
                    .collect();
                tasks_to_render.sort_by(|a, b| a.1.created_at.cmp(&b.1.created_at));
                
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (task_name, task) in &tasks_to_render {
                        self.render_task_card(ui, task_name, task);
                        ui.add_space(8.0);
                    }
                });
            }
        });
        
        // Reset confirmation dialog
        if self.show_reset_dialog {
            egui::Window::new("Confirm Reset")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
                .show(ctx, |ui| {
                    ui.label(&format!("Are you sure you want to reset the time for '{}'?", self.task_to_reset));
                    ui.add_space(10.0);
                    
                    ui.horizontal(|ui| {
                        if MaterialUI::material_button(ui, "Cancel", false).clicked() {
                            self.show_reset_dialog = false;
                        }
                        
                        ui.add_space(8.0);
                        
                        if MaterialUI::material_button(ui, "Reset", true).clicked() {
                            if let Some(task) = self.tasks.get_mut(&self.task_to_reset) {
                                task.reset();
                                // Auto-save when task is reset
                                self.save_tasks();
                            }
                            self.show_reset_dialog = false;
                        }
                    });
                });
        }

        // Delete confirmation dialog
        if self.show_delete_dialog {
            egui::Window::new("Confirm Delete")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
                .show(ctx, |ui| {
                    ui.label(&format!("Are you sure you want to delete the task '{}'?", self.task_to_delete));
                    ui.add_space(5.0);
                    ui.label(egui::RichText::new("This action cannot be undone.").color(egui::Color32::from_rgb(200, 50, 50)));
                    ui.add_space(10.0);
                    
                    ui.horizontal(|ui| {
                        if MaterialUI::material_button(ui, "Cancel", false).clicked() {
                            self.show_delete_dialog = false;
                        }
                        
                        ui.add_space(8.0);
                        
                        if MaterialUI::material_button(ui, "Delete", true).clicked() {
                            self.tasks.remove(&self.task_to_delete);
                            // Auto-save when task is deleted
                            self.save_tasks();
                            self.show_delete_dialog = false;
                        }
                    });
                });
        }
    }
}