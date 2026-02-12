#!/usr/bin/env bash
#
# Time Tracker - Linux Installation Script
# =========================================
# This script builds and installs the Time Tracker application on Linux systems.
#
# Usage:
#   ./install-linux.sh [command]
#
# Commands:
#   install      Build and install the application (default)
#   binary-only  Build and install just the binary (no desktop files, icons, etc.)
#   uninstall    Remove the application from the system
#   build        Build without installing
#   help         Show this help message
#

set -euo pipefail

# ============================================================================
# Configuration
# ============================================================================

APP_NAME="time-tracker"
APP_DISPLAY_NAME="Time Tracker"
APP_IDENTIFIER="com.timetracker.timetracker"
APP_DESCRIPTION="A Material Design Time Tracker built with Tauri"
APP_CATEGORIES="Utility;Office;ProjectManagement;"

# Installation paths
PREFIX="${PREFIX:-/usr/local}"
BIN_DIR="${PREFIX}/bin"
SHARE_DIR="${PREFIX}/share"
APPLICATIONS_DIR="${SHARE_DIR}/applications"
ICONS_DIR="${SHARE_DIR}/icons/hicolor"

# Build configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_DIR="${SCRIPT_DIR}/src-tauri/target/release"
BUNDLE_DIR="${BUILD_DIR}/bundle/deb"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# ============================================================================
# Helper Functions
# ============================================================================

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
}

die() {
    log_error "$1"
    exit 1
}

check_command() {
    if ! command -v "$1" &> /dev/null; then
        return 1
    fi
    return 0
}

check_root() {
    if [[ $EUID -ne 0 ]]; then
        return 1
    fi
    return 0
}

# ============================================================================
# Dependency Checks
# ============================================================================

check_dependencies() {
    log_info "Checking dependencies..."

    local missing_deps=()

    # Required tools
    if ! check_command "cargo"; then
        missing_deps+=("cargo (Rust toolchain)")
    fi

    if ! check_command "npm"; then
        missing_deps+=("npm (Node.js)")
    fi

    # Check for Tauri CLI
    if ! check_command "cargo-tauri" && ! cargo tauri --version &> /dev/null 2>&1; then
        log_warning "Tauri CLI not found. Will attempt to install via cargo."
    fi

    # Check for required system libraries (common Tauri dependencies on Linux)
    local required_libs=(
        "libwebkit2gtk-4.1"
        "libgtk-3"
    )

    # Check via pkg-config if available
    if check_command "pkg-config"; then
        for lib in "${required_libs[@]}"; do
            if ! pkg-config --exists "${lib}" 2>/dev/null; then
                # Try alternative names
                case "$lib" in
                    "libwebkit2gtk-4.1")
                        if ! pkg-config --exists "webkit2gtk-4.1" 2>/dev/null && \
                           ! pkg-config --exists "webkit2gtk-4.0" 2>/dev/null; then
                            missing_deps+=("$lib (webkit2gtk development package)")
                        fi
                        ;;
                    "libgtk-3")
                        if ! pkg-config --exists "gtk+-3.0" 2>/dev/null; then
                            missing_deps+=("$lib (GTK3 development package)")
                        fi
                        ;;
                esac
            fi
        done
    else
        log_warning "pkg-config not found. Cannot verify system library dependencies."
    fi

    if [[ ${#missing_deps[@]} -gt 0 ]]; then
        log_error "Missing dependencies:"
        for dep in "${missing_deps[@]}"; do
            echo "  - $dep"
        done
        echo ""
        echo "On Debian/Ubuntu, install with:"
        echo "  sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev build-essential curl wget file libssl-dev libayatana-appindicator3-dev librsvg2-dev"
        echo ""
        echo "On Fedora, install with:"
        echo "  sudo dnf install webkit2gtk4.1-devel gtk3-devel openssl-devel libappindicator-gtk3-devel librsvg2-devel"
        echo ""
        echo "On Arch Linux, install with:"
        echo "  sudo pacman -S webkit2gtk-4.1 gtk3 openssl libappindicator-gtk3 librsvg"
        echo ""
        die "Please install the missing dependencies and try again."
    fi

    log_success "All dependencies satisfied."
}

# ============================================================================
# Build Functions
# ============================================================================

install_tauri_cli() {
    if ! cargo tauri --version &> /dev/null 2>&1; then
        log_info "Installing Tauri CLI..."
        cargo install tauri-cli || die "Failed to install Tauri CLI"
        log_success "Tauri CLI installed."
    fi
}

install_npm_dependencies() {
    log_info "Installing npm dependencies..."
    cd "${SCRIPT_DIR}"
    npm install || die "Failed to install npm dependencies"
    log_success "npm dependencies installed."
}

build_application() {
    log_info "Building application in release mode..."
    cd "${SCRIPT_DIR}"

    # Build with Tauri
    cargo tauri build || die "Build failed"

    log_success "Application built successfully."
}

build_binary_only() {
    log_info "Building binary only (no bundling)..."
    cd "${SCRIPT_DIR}"

    # Build with Tauri but skip all bundling (no deb, rpm, AppImage, etc.)
    cargo tauri build --no-bundle || die "Build failed"

    log_success "Binary built successfully."
}

# ============================================================================
# Installation Functions
# ============================================================================

find_built_binary() {
    # Look for the binary in various possible locations
    local possible_paths=(
        "${BUILD_DIR}/${APP_NAME}"
        "${BUILD_DIR}/time-tracker-tauri"
        "${SCRIPT_DIR}/src-tauri/target/release/${APP_NAME}"
        "${SCRIPT_DIR}/src-tauri/target/release/time-tracker-tauri"
    )

    for path in "${possible_paths[@]}"; do
        if [[ -f "$path" && -x "$path" ]]; then
            echo "$path"
            return 0
        fi
    done

    # Try to find any executable in the release directory
    local found
    found=$(find "${SCRIPT_DIR}/src-tauri/target/release" -maxdepth 1 -type f -executable -name "*time*tracker*" 2>/dev/null | head -n1)
    if [[ -n "$found" ]]; then
        echo "$found"
        return 0
    fi

    return 1
}

find_icon() {
    local size="$1"
    local icon_paths=(
        "${SCRIPT_DIR}/src-tauri/icons/${size}x${size}.png"
        "${SCRIPT_DIR}/src-tauri/icons/icon.png"
    )

    for path in "${icon_paths[@]}"; do
        if [[ -f "$path" ]]; then
            echo "$path"
            return 0
        fi
    done

    return 1
}

create_desktop_file() {
    local desktop_file="$1"
    local binary_path="$2"

    cat > "$desktop_file" << EOF
[Desktop Entry]
Name=${APP_DISPLAY_NAME}
Comment=${APP_DESCRIPTION}
Exec=${binary_path}
Icon=${APP_NAME}
Terminal=false
Type=Application
Categories=${APP_CATEGORIES}
StartupWMClass=${APP_NAME}
Keywords=time;tracker;productivity;tasks;
EOF
}

install_application() {
    log_info "Installing application..."

    # Check for root/sudo for system-wide install
    local use_sudo=""
    if ! check_root; then
        if [[ "$PREFIX" == "/usr/local" || "$PREFIX" == "/usr" ]]; then
            log_info "Root privileges required for installation to ${PREFIX}"
            use_sudo="sudo"
        fi
    fi

    # Find the built binary
    local binary_path
    binary_path=$(find_built_binary) || die "Could not find built binary. Did the build succeed?"
    log_info "Found binary: ${binary_path}"

    # Create directories
    log_info "Creating installation directories..."
    $use_sudo mkdir -p "${BIN_DIR}"
    $use_sudo mkdir -p "${APPLICATIONS_DIR}"

    # Install binary
    log_info "Installing binary to ${BIN_DIR}/${APP_NAME}..."
    $use_sudo cp "${binary_path}" "${BIN_DIR}/${APP_NAME}"
    $use_sudo chmod 755 "${BIN_DIR}/${APP_NAME}"

    # Install desktop file
    log_info "Installing desktop file..."
    local temp_desktop
    temp_desktop=$(mktemp)
    create_desktop_file "$temp_desktop" "${BIN_DIR}/${APP_NAME}"
    $use_sudo mv "$temp_desktop" "${APPLICATIONS_DIR}/${APP_NAME}.desktop"
    $use_sudo chmod 644 "${APPLICATIONS_DIR}/${APP_NAME}.desktop"

    # Install icons
    log_info "Installing icons..."
    local icon_sizes=(16 24 32 48 64 128 256 512)
    for size in "${icon_sizes[@]}"; do
        local icon_src
        icon_src=$(find_icon "$size") || continue

        local icon_dest_dir="${ICONS_DIR}/${size}x${size}/apps"
        $use_sudo mkdir -p "${icon_dest_dir}"
        $use_sudo cp "${icon_src}" "${icon_dest_dir}/${APP_NAME}.png"
        $use_sudo chmod 644 "${icon_dest_dir}/${APP_NAME}.png"
    done

    # Also install a scalable icon if we have a large PNG
    local large_icon
    large_icon=$(find_icon 512) || large_icon=$(find_icon 256) || large_icon=$(find_icon 128)
    if [[ -n "$large_icon" ]]; then
        $use_sudo mkdir -p "${ICONS_DIR}/scalable/apps"
        $use_sudo cp "${large_icon}" "${ICONS_DIR}/scalable/apps/${APP_NAME}.png"
    fi

    # Update desktop database
    if check_command "update-desktop-database"; then
        log_info "Updating desktop database..."
        $use_sudo update-desktop-database "${APPLICATIONS_DIR}" 2>/dev/null || true
    fi

    # Update icon cache
    if check_command "gtk-update-icon-cache"; then
        log_info "Updating icon cache..."
        $use_sudo gtk-update-icon-cache -f -t "${ICONS_DIR}" 2>/dev/null || true
    fi

    log_success "Installation complete!"
    echo ""
    echo "The application has been installed to: ${BIN_DIR}/${APP_NAME}"
    echo "You can now run it from the command line with: ${APP_NAME}"
    echo "Or find it in your application menu as '${APP_DISPLAY_NAME}'"
}

install_binary_only() {
    log_info "Installing binary only..."

    # Check for root/sudo for system-wide install
    local use_sudo=""
    if ! check_root; then
        if [[ "$PREFIX" == "/usr/local" || "$PREFIX" == "/usr" ]]; then
            log_info "Root privileges required for installation to ${PREFIX}"
            use_sudo="sudo"
        fi
    fi

    # Find the built binary
    local binary_path
    binary_path=$(find_built_binary) || die "Could not find built binary. Did the build succeed?"
    log_info "Found binary: ${binary_path}"

    # Create bin directory and install binary
    $use_sudo mkdir -p "${BIN_DIR}"
    log_info "Installing binary to ${BIN_DIR}/${APP_NAME}..."
    $use_sudo cp "${binary_path}" "${BIN_DIR}/${APP_NAME}"
    $use_sudo chmod 755 "${BIN_DIR}/${APP_NAME}"

    log_success "Binary-only installation complete!"
    echo ""
    echo "The binary has been installed to: ${BIN_DIR}/${APP_NAME}"
    echo "You can run it from the command line with: ${APP_NAME}"
}

# ============================================================================
# Uninstall Functions
# ============================================================================

uninstall_application() {
    log_info "Uninstalling ${APP_DISPLAY_NAME}..."

    # Check for root/sudo for system-wide uninstall
    local use_sudo=""
    if ! check_root; then
        if [[ "$PREFIX" == "/usr/local" || "$PREFIX" == "/usr" ]]; then
            log_info "Root privileges required for uninstallation from ${PREFIX}"
            use_sudo="sudo"
        fi
    fi

    # Remove binary
    if [[ -f "${BIN_DIR}/${APP_NAME}" ]]; then
        log_info "Removing binary..."
        $use_sudo rm -f "${BIN_DIR}/${APP_NAME}"
    fi

    # Remove desktop file
    if [[ -f "${APPLICATIONS_DIR}/${APP_NAME}.desktop" ]]; then
        log_info "Removing desktop file..."
        $use_sudo rm -f "${APPLICATIONS_DIR}/${APP_NAME}.desktop"
    fi

    # Remove icons
    log_info "Removing icons..."
    local icon_sizes=(16 24 32 48 64 128 256 512 scalable)
    for size in "${icon_sizes[@]}"; do
        local icon_path
        if [[ "$size" == "scalable" ]]; then
            icon_path="${ICONS_DIR}/${size}/apps/${APP_NAME}.png"
        else
            icon_path="${ICONS_DIR}/${size}x${size}/apps/${APP_NAME}.png"
        fi
        if [[ -f "$icon_path" ]]; then
            $use_sudo rm -f "$icon_path"
        fi
    done

    # Update desktop database
    if check_command "update-desktop-database"; then
        log_info "Updating desktop database..."
        $use_sudo update-desktop-database "${APPLICATIONS_DIR}" 2>/dev/null || true
    fi

    # Update icon cache
    if check_command "gtk-update-icon-cache"; then
        log_info "Updating icon cache..."
        $use_sudo gtk-update-icon-cache -f -t "${ICONS_DIR}" 2>/dev/null || true
    fi

    log_success "Uninstallation complete!"
    echo ""
    echo "Note: User configuration data in ~/.config/ has been preserved."
    echo "To remove all data, delete: ~/.config/time_tracker_tauri_data.json"
}

# ============================================================================
# Help
# ============================================================================

show_help() {
    cat << EOF
${APP_DISPLAY_NAME} - Linux Installation Script

Usage:
    $0 [command]

Commands:
    install      Build and install the application (default)
    binary-only  Build and install just the binary (no desktop files, icons, etc.)
    uninstall    Remove the application from the system
    build        Build without installing
    help         Show this help message

Environment Variables:
    PREFIX      Installation prefix (default: /usr/local)
                Example: PREFIX=~/.local ./install-linux.sh install

Examples:
    # Install system-wide (requires sudo)
    ./install-linux.sh install

    # Install just the binary (no AppImage, desktop files, or icons)
    ./install-linux.sh binary-only

    # Install for current user only
    PREFIX=~/.local ./install-linux.sh install

    # Uninstall
    ./install-linux.sh uninstall

    # Build only (no installation)
    ./install-linux.sh build

EOF
}

# ============================================================================
# Main
# ============================================================================

main() {
    local command="${1:-install}"

    case "$command" in
        install)
            echo "========================================"
            echo "  ${APP_DISPLAY_NAME} - Installation"
            echo "========================================"
            echo ""
            check_dependencies
            install_tauri_cli
            install_npm_dependencies
            build_application
            install_application
            ;;
        binary-only|binary)
            echo "========================================"
            echo "  ${APP_DISPLAY_NAME} - Binary Only"
            echo "========================================"
            echo ""
            check_dependencies
            install_tauri_cli
            install_npm_dependencies
            build_binary_only
            install_binary_only
            ;;
        uninstall)
            echo "========================================"
            echo "  ${APP_DISPLAY_NAME} - Uninstallation"
            echo "========================================"
            echo ""
            uninstall_application
            ;;
        build)
            echo "========================================"
            echo "  ${APP_DISPLAY_NAME} - Build Only"
            echo "========================================"
            echo ""
            check_dependencies
            install_tauri_cli
            install_npm_dependencies
            build_application
            log_success "Build complete. Binary located at: $(find_built_binary)"
            ;;
        help|--help|-h)
            show_help
            ;;
        *)
            log_error "Unknown command: $command"
            echo ""
            show_help
            exit 1
            ;;
    esac
}

main "$@"
