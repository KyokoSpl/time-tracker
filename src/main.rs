mod task;
mod persistence;
mod ui;
mod app;

use app::TimeTrackerApp;

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
