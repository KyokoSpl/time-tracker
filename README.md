# Time Tracker

A beautiful, modern time tracking application built with **Tauri** (Rust backend) and **BeerCSS** (Material Design 3 frontend), featuring the **One Dark** color scheme.

## Features

- **🎨 One Dark Theme**: Beautiful dark theme inspired by the popular Atom editor theme
- **📱 Material Design 3**: Modern UI following BeerCSS/Material Design 3 guidelines
- **⏱️ Task Time Tracking**: Start, stop, and track time for multiple tasks
- **💾 Persistent Storage**: Tasks automatically saved and restored between sessions
- **📤 Export to TXT**: Export all tasks and times to a text file
- **🌙 Dark/Light Toggle**: Switch between dark and light themes
- **🔄 Real-time Updates**: Live time display for running tasks
- **✅ Confirmation Dialogs**: Prevent accidental resets and deletions
- **🖥️ Cross-Platform**: Runs on Linux, Windows, and macOS

## Tech Stack

### Backend (Rust)
- **Tauri 2.0**: Secure, lightweight desktop app framework
- **Serde**: JSON serialization for persistence
- **Chrono**: Date and time handling

### Frontend
- **BeerCSS 4.0**: Material Design 3 CSS framework
- **Vanilla JavaScript**: No framework dependencies
- **Material Symbols**: Google Material icons

## Prerequisites

Before building the project, ensure you have the following installed:

1. **Rust** (latest stable version)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Node.js** (v18 or later)
   ```bash
   # Using nvm (recommended)
   nvm install 18
   nvm use 18
   ```

3. **System Dependencies** (Linux only)
   ```bash
   # Debian/Ubuntu
   sudo apt update
   sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file \
       libssl-dev libayatana-appindicator3-dev librsvg2-dev

   # Fedora
   sudo dnf install webkit2gtk4.1-devel openssl-devel curl wget file \
       libappindicator-gtk3-devel librsvg2-devel

   # Arch Linux
   sudo pacman -S webkit2gtk-4.1 base-devel curl wget file openssl \
       appmenu-gtk-module libappindicator-gtk3 librsvg
   ```

## Installation

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd time-tracker
   ```

2. **Install Node.js dependencies**
   ```bash
   npm install
   ```

3. **Build the application**
   ```bash
   npm run build
   ```

## Development

### Run in development mode

```bash
npm run dev
```

This will start the application with hot-reloading enabled for the frontend.

### Project Structure

```
time-tracker/
├── src/                        # Frontend source
│   ├── index.html             # Main HTML with BeerCSS
│   ├── styles.css             # One Dark theme CSS
│   ├── app.js                 # JavaScript application logic
│   ├── beer.min.css           # BeerCSS framework
│   └── beer.min.js            # BeerCSS JavaScript
├── src-tauri/                 # Rust backend
│   ├── src/
│   │   ├── main.rs           # Entry point
│   │   ├── lib.rs            # Library with Tauri setup
│   │   ├── commands.rs       # Tauri IPC commands
│   │   ├── state.rs          # Application state management
│   │   ├── task.rs           # Task domain model
│   │   └── persistence.rs    # File persistence
│   ├── capabilities/         # Tauri security capabilities
│   ├── icons/                # Application icons
│   ├── Cargo.toml            # Rust dependencies
│   └── tauri.conf.json       # Tauri configuration
├── package.json              # Node.js configuration
├── LICENSE                   # MIT License
└── README.md                 # This file
```

## Usage

### Adding Tasks
1. Enter a task name in the input field
2. Click "Add Task" or press Enter
3. The task appears as a card below

### Time Tracking
- Click the **play button** (▶️) to start tracking time
- Click the **pause button** (⏸️) to stop tracking
- Time accumulates across multiple start/stop sessions

### Managing Tasks
- **Reset**: Click the replay icon to reset a task's time to 00:00:00
- **Delete**: Click the trash icon to permanently remove a task
- Both actions require confirmation

### Exporting Data
1. Click the "Export" button in the header
2. Choose a location to save the file
3. Tasks are exported in a readable text format

### Theme Toggle
- Click the sun/moon icon in the header to switch between dark and light themes

## Data Storage

Tasks are automatically saved to:
- **Linux**: `~/.config/time_tracker_tauri_data.json`
- **macOS**: `~/Library/Application Support/time_tracker_tauri_data.json`
- **Windows**: `%APPDATA%\time_tracker_tauri_data.json`

## Architecture

The application follows a clean separation of concerns:

### Backend (Rust)
- **Domain Layer** (`task.rs`): Task entity with business logic
- **Persistence Layer** (`persistence.rs`): File I/O operations
- **State Management** (`state.rs`): Thread-safe application state
- **API Layer** (`commands.rs`): Tauri commands for frontend communication

### Frontend (JavaScript)
- **UI Rendering**: Pure DOM manipulation with BeerCSS components
- **State Sync**: Polls backend for running task updates
- **Event Handling**: User interactions trigger Tauri IPC calls

## One Dark Color Scheme

The application uses the One Dark color palette:

| Color      | Hex       | Usage                    |
|------------|-----------|--------------------------|
| Background | `#282c34` | Main background          |
| Foreground | `#abb2bf` | Text and icons           |
| Blue       | `#61afef` | Primary, active elements |
| Purple     | `#c678dd` | Secondary accents        |
| Green      | `#98c379` | Success, start button    |
| Red        | `#e06c75` | Error, stop/delete       |
| Yellow     | `#e5c07b` | Warnings                 |
| Cyan       | `#56b6c2` | Tertiary accents         |

## Building for Production

### Linux (AppImage, DEB)
```bash
npm run build
# Output: src-tauri/target/release/bundle/
```

### Windows (MSI, EXE)
```bash
npm run build
# Output: src-tauri\target\release\bundle\
```

### macOS (DMG, APP)
```bash
npm run build
# Output: src-tauri/target/release/bundle/
```

## Troubleshooting

### "Failed to acquire lock" error
The application state may be corrupted. Delete the data file and restart:
```bash
rm ~/.config/time_tracker_tauri_data.json
```

### Build fails on Linux
Ensure all system dependencies are installed (see Prerequisites).

### Tauri API not available
Make sure `withGlobalTauri` is set to `true` in `src-tauri/tauri.conf.json`.

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Tauri](https://tauri.app/) - Desktop app framework
- [BeerCSS](https://www.beercss.com/) - Material Design 3 CSS framework
- [One Dark](https://github.com/atom/atom/tree/master/packages/one-dark-syntax) - Color scheme inspiration
- [Material Symbols](https://fonts.google.com/icons) - Icon font