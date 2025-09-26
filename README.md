# Material Design Time Tracker

A beautiful task time tracking application built with egui and Material Design 3 principles in Rust.

## Features

- **✨ Material Design 3 UI**: Modern, clean interface with Material Design styling
- **📝 Task Input**: Add new tasks by typing the task name and clicking "Add Task" or pressing Enter
- **⏱️ Running Time Display**: Real-time display of elapsed time for each task in HH:MM:SS format
- **🎯 Stopwatch Functionality**: Start/Stop buttons for each task to control time tracking
- **➕ Add Task Button**: Create new tasks with custom names
- **🔄 Reset Button**: Reset the time for any task (with confirmation dialog)
- **💾 Export to TXT**: Export all tasks and their times to a text file
- **🌙 Dark/Light Theme**: Toggle between dark and light themes
- **🎨 Material Cards**: Each task displayed in elevated Material Design cards
- **🔄 Running Indicators**: Visual spinner for currently running tasks

## Building and Running

### Prerequisites
- Rust (latest stable version)
- No additional system dependencies required! (egui handles all UI rendering)

### Build
```bash
cargo build
```

### Run
```bash
cargo run
```

## Usage

1. **Adding Tasks**: 
   - Type a task name in the input field
   - Click "Add Task" or press Enter
   - The task will appear as a Material Design card below

2. **Time Tracking**:
   - Click "Start" to begin tracking time for a task
   - A spinner appears and the button changes to "Stop"
   - Click "Stop" to pause time tracking
   - Time continues to accumulate across multiple start/stop sessions

3. **Theme Switching**:
   - Click the sun/moon icon in the top bar to toggle between light and dark themes
   - The Material Design colors adapt automatically

4. **Resetting Tasks**:
   - Click "Reset" next to any task
   - Confirm the action in the modal dialog
   - The task time will be reset to 00:00:00

5. **Exporting Data**:
   - Click "Export" in the top bar
   - Choose a location to save the export file
   - The file contains all tasks with their total times and creation dates

## File Structure

- `src/main.rs`: Main application code with egui interface and time tracking logic
- `Cargo.toml`: Project dependencies and metadata

## Dependencies

- **eframe**: egui framework for cross-platform GUI applications
- **egui**: Immediate mode GUI library
- **chrono**: Date and time handling
- **serde**: Serialization framework
- **serde_json**: JSON serialization support
- **rfd**: Native file dialogs

## Features Highlights

### Material Design 3 Elements
- **Elevated Cards**: Each task is displayed in a Material Design card with subtle shadows
- **Material Buttons**: Rounded buttons with proper Material Design styling
- **Color Scheme**: Adaptive colors that work in both light and dark themes
- **Typography**: Material Design typography with proper text sizing and weights
- **Spacing**: Consistent Material Design spacing throughout the interface

### Cross-Platform Compatibility
- Runs on Linux, Windows, and macOS
- No system UI dependencies - all rendering handled by egui
- Consistent appearance across all platforms

### Performance
- Efficient immediate mode rendering
- Smooth 10fps updates for time display
- Minimal memory usage
- Fast startup time

## Notes

- Tasks are kept in memory during the application session
- Export files contain task name, total time, running status, and creation timestamp
- The application updates the display every 100ms for smooth time tracking
- Confirmation dialogs prevent accidental time resets
- Material Design principles ensure a consistent, accessible user interface
