use eframe::egui::{self, Color32, Rounding, Stroke, Vec2};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use chrono::{DateTime, Local};
use std::fs::File;
use std::io::Write;

#[derive(Clone, Debug)]
struct Task {
    name: String,
    total_time: Duration,
    start_time: Option<Instant>,
    is_running: bool,
    created_at: DateTime<Local>,
}

impl Task {
    fn new(name: String) -> Self {
        Task {
            name,
            total_time: Duration::new(0, 0),
            start_time: None,
            is_running: false,
            created_at: Local::now(),
        }
    }
    
    fn start(&mut self) {
        if !self.is_running {
            self.start_time = Some(Instant::now());
            self.is_running = true;
        }
    }
    
    fn stop(&mut self) {
        if self.is_running {
            if let Some(start) = self.start_time {
                self.total_time += start.elapsed();
            }
            self.is_running = false;
            self.start_time = None;
        }
    }
    
    fn reset(&mut self) {
        self.stop();
        self.total_time = Duration::new(0, 0);
    }
    
    fn get_current_time(&self) -> Duration {
        let mut current_time = self.total_time;
        if self.is_running {
            if let Some(start) = self.start_time {
                current_time += start.elapsed();
            }
        }
        current_time
    }
    
    fn format_duration(duration: Duration) -> String {
        let total_seconds = duration.as_secs();
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
}

#[derive(Default)]
struct TimeTrackerApp {
    tasks: HashMap<String, Task>,
    new_task_name: String,
    show_reset_dialog: bool,
    task_to_reset: String,
    dark_mode: bool,
}

impl TimeTrackerApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Configure egui style for Material Design look
        let mut style = egui::Style::default();
        
        // Material Design color scheme
        style.visuals.button_frame = true;
        style.visuals.collapsing_header_frame = true;
        style.spacing.button_padding = Vec2::new(16.0, 8.0);
        style.spacing.indent = 20.0;
        style.spacing.item_spacing = Vec2::new(8.0, 8.0);
        
        // Material Design rounding
        style.visuals.widgets.noninteractive.rounding = Rounding::same(8.0);
        style.visuals.widgets.inactive.rounding = Rounding::same(8.0);
        style.visuals.widgets.hovered.rounding = Rounding::same(8.0);
        style.visuals.widgets.active.rounding = Rounding::same(8.0);
        
        _cc.egui_ctx.set_style(style);
        
        Self::default()
    }
    
    fn export_to_txt(&self) {
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
                    
                    let mut sorted_tasks: Vec<_> = self.tasks.iter().collect();
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
    
    fn toggle_theme(&mut self, ctx: &egui::Context) {
        self.dark_mode = !self.dark_mode;
        
        let mut style = ctx.style().as_ref().clone();
        if self.dark_mode {
            style.visuals = egui::Visuals::dark();
        } else {
            style.visuals = egui::Visuals::light();
        }
        
        // Keep Material Design styling
        style.visuals.button_frame = true;
        style.visuals.collapsing_header_frame = true;
        style.spacing.button_padding = Vec2::new(16.0, 8.0);
        style.spacing.item_spacing = Vec2::new(8.0, 8.0);
        
        style.visuals.widgets.noninteractive.rounding = Rounding::same(8.0);
        style.visuals.widgets.inactive.rounding = Rounding::same(8.0);
        style.visuals.widgets.hovered.rounding = Rounding::same(8.0);
        style.visuals.widgets.active.rounding = Rounding::same(8.0);
        
        ctx.set_style(style);
    }
    
    fn material_button(ui: &mut egui::Ui, text: &str, filled: bool) -> egui::Response {
        let button = if filled {
            egui::Button::new(text)
                .fill(Color32::from_rgb(103, 80, 164)) // Material Purple
                .stroke(Stroke::NONE)
        } else {
            egui::Button::new(text)
                .fill(Color32::TRANSPARENT)
                .stroke(Stroke::new(1.0, Color32::from_rgb(103, 80, 164)))
        };
        
        ui.add_sized([80.0, 32.0], button)
    }
    
    fn material_card<R>(
        ui: &mut egui::Ui,
        elevated: bool,
        content: impl FnOnce(&mut egui::Ui) -> R,
    ) -> R {
        let frame = if elevated {
            egui::Frame::default()
                .fill(ui.visuals().panel_fill)
                .stroke(Stroke::NONE)
                .rounding(Rounding::same(12.0))
                .shadow(egui::epaint::Shadow {
                    offset: Vec2::new(0.0, 2.0),
                    blur: 4.0,
                    spread: 0.0,
                    color: Color32::from_black_alpha(25),
                })
                .inner_margin(16.0)
        } else {
            egui::Frame::default()
                .fill(ui.visuals().panel_fill)
                .stroke(Stroke::new(1.0, ui.visuals().widgets.noninteractive.bg_stroke.color))
                .rounding(Rounding::same(12.0))
                .inner_margin(16.0)
        };
        
        frame.show(ui, content).inner
    }
    
    fn render_task_card(&mut self, ui: &mut egui::Ui, task_name: &str, task: &Task) {
        let task_name_owned = task_name.to_string();
        let (reset_clicked, start_stop_clicked) = Self::material_card(ui, true, |ui| {
            ui.horizontal(|ui| {
                // Task name
                ui.label(egui::RichText::new(task_name).size(16.0).strong());
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Reset button
                    let reset_clicked = Self::material_button(ui, "Reset", false).clicked();
                    
                    ui.add_space(8.0);
                    
                    // Start/Stop button
                    let button_text = if task.is_running { "Stop" } else { "Start" };
                    let start_stop_clicked = Self::material_button(ui, button_text, task.is_running).clicked();
                    
                    ui.add_space(8.0);
                    
                    // Running indicator
                    if task.is_running {
                        ui.spinner();
                        ui.add_space(8.0);
                    }
                    
                    // Time display
                    let time_str = Task::format_duration(task.get_current_time());
                    ui.label(egui::RichText::new(&time_str).size(18.0).monospace());
                    
                    (reset_clicked, start_stop_clicked)
                }).inner
            }).inner
        });
        
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
            }
        }
    }
}

impl eframe::App for TimeTrackerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Request repaint for smooth time updates
        ctx.request_repaint_after(Duration::from_millis(100));
        
        // Top app bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("🕐 Time Tracker");
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Export button
                    if Self::material_button(ui, "Export", false).clicked() {
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
            let add_clicked = Self::material_card(ui, false, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Add New Task:").size(14.0));
                    
                    let response = ui.add(
                        egui::TextEdit::singleline(&mut self.new_task_name)
                            .hint_text("Enter task name")
                            .desired_width(200.0)
                    );
                    
                    let add_clicked = Self::material_button(ui, "Add Task", true).clicked();
                    
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
                        if Self::material_button(ui, "Cancel", false).clicked() {
                            self.show_reset_dialog = false;
                        }
                        
                        ui.add_space(8.0);
                        
                        if Self::material_button(ui, "Reset", true).clicked() {
                            if let Some(task) = self.tasks.get_mut(&self.task_to_reset) {
                                task.reset();
                            }
                            self.show_reset_dialog = false;
                        }
                    });
                });
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([600.0, 400.0])
            .with_title("Time Tracker"),
        ..Default::default()
    };
    
    eframe::run_native(
        "Time Tracker",
        options,
        Box::new(|cc| Ok(Box::new(TimeTrackerApp::new(cc)))),
    )
}
