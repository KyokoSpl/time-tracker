#Requires -Version 5.1
<#
.SYNOPSIS
    Time Tracker Installation Script for Windows

.DESCRIPTION
    Builds and installs the Time Tracker Tauri application on Windows.
    Supports install, uninstall, and build-only operations.

.PARAMETER Action
    The action to perform: install, uninstall, or build
    Default: install

.PARAMETER InstallDir
    Custom installation directory
    Default: $env:LOCALAPPDATA\Programs\TimeTracker

.EXAMPLE
    .\install-windows.ps1
    Builds and installs the application

.EXAMPLE
    .\install-windows.ps1 -Action uninstall
    Uninstalls the application

.EXAMPLE
    .\install-windows.ps1 -Action build
    Only builds the application without installing
#>

param(
    [ValidateSet("install", "uninstall", "build")]
    [string]$Action = "install",

    [string]$InstallDir = "$env:LOCALAPPDATA\Programs\TimeTracker"
)

$ErrorActionPreference = "Stop"

# Application metadata
$AppName = "Time Tracker"
$AppExecutable = "time-tracker-tauri.exe"
$AppIdentifier = "com.timetracker.app"
$StartMenuFolder = "$env:APPDATA\Microsoft\Windows\Start Menu\Programs"
$DesktopShortcut = "$env:USERPROFILE\Desktop\$AppName.lnk"

function Write-Header {
    param([string]$Message)
    Write-Host ""
    Write-Host "========================================" -ForegroundColor Cyan
    Write-Host " $Message" -ForegroundColor Cyan
    Write-Host "========================================" -ForegroundColor Cyan
    Write-Host ""
}

function Write-Step {
    param([string]$Message)
    Write-Host "[*] $Message" -ForegroundColor Green
}

function Write-Warning {
    param([string]$Message)
    Write-Host "[!] $Message" -ForegroundColor Yellow
}

function Write-Error {
    param([string]$Message)
    Write-Host "[X] $Message" -ForegroundColor Red
}

function Test-Command {
    param([string]$Command)
    $null = Get-Command $Command -ErrorAction SilentlyContinue
    return $?
}

function Test-Prerequisites {
    Write-Header "Checking Prerequisites"

    $missing = @()

    # Check for Rust/Cargo
    if (-not (Test-Command "cargo")) {
        $missing += "Rust/Cargo (https://rustup.rs)"
    } else {
        $rustVersion = & rustc --version 2>&1
        Write-Step "Rust: $rustVersion"
    }

    # Check for Node.js/npm
    if (-not (Test-Command "npm")) {
        $missing += "Node.js/npm (https://nodejs.org)"
    } else {
        $nodeVersion = & node --version 2>&1
        Write-Step "Node.js: $nodeVersion"
    }

    # Check for Tauri CLI
    $tauriInstalled = $false
    if (Test-Command "cargo-tauri") {
        $tauriInstalled = $true
    } else {
        # Check if tauri is available via cargo
        $tauriCheck = & cargo tauri --version 2>&1
        if ($LASTEXITCODE -eq 0) {
            $tauriInstalled = $true
        }
    }

    if (-not $tauriInstalled) {
        Write-Warning "Tauri CLI not found. It will be installed automatically."
    } else {
        $tauriVersion = & cargo tauri --version 2>&1
        Write-Step "Tauri CLI: $tauriVersion"
    }

    if ($missing.Count -gt 0) {
        Write-Error "Missing required dependencies:"
        foreach ($dep in $missing) {
            Write-Host "  - $dep" -ForegroundColor Red
        }
        Write-Host ""
        Write-Host "Please install the missing dependencies and try again." -ForegroundColor Red
        exit 1
    }

    Write-Step "All prerequisites satisfied"
}

function Install-TauriCLI {
    Write-Step "Installing Tauri CLI..."
    & cargo install tauri-cli
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Failed to install Tauri CLI"
        exit 1
    }
}

function Build-Application {
    Write-Header "Building Application"

    $projectRoot = $PSScriptRoot

    # Install npm dependencies
    Write-Step "Installing npm dependencies..."
    Push-Location $projectRoot
    try {
        & npm install
        if ($LASTEXITCODE -ne 0) {
            Write-Error "Failed to install npm dependencies"
            exit 1
        }

        # Check/install Tauri CLI
        $tauriCheck = & cargo tauri --version 2>&1
        if ($LASTEXITCODE -ne 0) {
            Install-TauriCLI
        }

        # Build the application
        Write-Step "Building Tauri application (this may take a while)..."
        & cargo tauri build
        if ($LASTEXITCODE -ne 0) {
            Write-Error "Failed to build application"
            exit 1
        }
    }
    finally {
        Pop-Location
    }

    Write-Step "Build completed successfully"
}

function Get-BuildArtifactPath {
    $projectRoot = $PSScriptRoot
    $releasePath = Join-Path $projectRoot "src-tauri\target\release"
    $bundlePath = Join-Path $projectRoot "src-tauri\target\release\bundle"

    # Check for MSI installer first
    $msiPath = Get-ChildItem -Path "$bundlePath\msi\*.msi" -ErrorAction SilentlyContinue | Select-Object -First 1
    if ($msiPath) {
        return @{
            Type = "msi"
            Path = $msiPath.FullName
        }
    }

    # Check for NSIS installer
    $nsisPath = Get-ChildItem -Path "$bundlePath\nsis\*.exe" -ErrorAction SilentlyContinue | Select-Object -First 1
    if ($nsisPath) {
        return @{
            Type = "nsis"
            Path = $nsisPath.FullName
        }
    }

    # Fall back to raw executable
    $exePath = Join-Path $releasePath $AppExecutable
    if (Test-Path $exePath) {
        return @{
            Type = "exe"
            Path = $exePath
        }
    }

    return $null
}

function New-Shortcut {
    param(
        [string]$ShortcutPath,
        [string]$TargetPath,
        [string]$Description = "",
        [string]$IconPath = ""
    )

    $shell = New-Object -ComObject WScript.Shell
    $shortcut = $shell.CreateShortcut($ShortcutPath)
    $shortcut.TargetPath = $TargetPath
    $shortcut.Description = $Description
    if ($IconPath -and (Test-Path $IconPath)) {
        $shortcut.IconLocation = $IconPath
    }
    $shortcut.Save()
}

function Install-Application {
    Write-Header "Installing Application"

    $artifact = Get-BuildArtifactPath

    if (-not $artifact) {
        Write-Error "No build artifacts found. Please run build first."
        exit 1
    }

    switch ($artifact.Type) {
        "msi" {
            Write-Step "Installing via MSI installer..."
            Write-Step "Artifact: $($artifact.Path)"
            Start-Process msiexec.exe -ArgumentList "/i `"$($artifact.Path)`" /passive" -Wait
            Write-Step "MSI installation completed"
        }
        "nsis" {
            Write-Step "Installing via NSIS installer..."
            Write-Step "Artifact: $($artifact.Path)"
            Start-Process -FilePath $artifact.Path -ArgumentList "/S" -Wait
            Write-Step "NSIS installation completed"
        }
        "exe" {
            Write-Step "Installing executable manually..."

            # Create installation directory
            if (-not (Test-Path $InstallDir)) {
                New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
                Write-Step "Created installation directory: $InstallDir"
            }

            # Copy executable
            $destExe = Join-Path $InstallDir $AppExecutable
            Copy-Item -Path $artifact.Path -Destination $destExe -Force
            Write-Step "Copied executable to: $destExe"

            # Create Start Menu shortcut
            $startMenuShortcut = Join-Path $StartMenuFolder "$AppName.lnk"
            New-Shortcut -ShortcutPath $startMenuShortcut -TargetPath $destExe -Description $AppName
            Write-Step "Created Start Menu shortcut"

            # Create Desktop shortcut
            New-Shortcut -ShortcutPath $DesktopShortcut -TargetPath $destExe -Description $AppName
            Write-Step "Created Desktop shortcut"

            # Add to PATH (optional - user level)
            $userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
            if ($userPath -notlike "*$InstallDir*") {
                [Environment]::SetEnvironmentVariable("PATH", "$userPath;$InstallDir", "User")
                Write-Step "Added installation directory to user PATH"
            }

            # Create uninstall registry entry
            $uninstallKey = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\$AppIdentifier"
            if (-not (Test-Path $uninstallKey)) {
                New-Item -Path $uninstallKey -Force | Out-Null
            }
            Set-ItemProperty -Path $uninstallKey -Name "DisplayName" -Value $AppName
            Set-ItemProperty -Path $uninstallKey -Name "UninstallString" -Value "powershell.exe -ExecutionPolicy Bypass -File `"$PSScriptRoot\install-windows.ps1`" -Action uninstall"
            Set-ItemProperty -Path $uninstallKey -Name "InstallLocation" -Value $InstallDir
            Set-ItemProperty -Path $uninstallKey -Name "Publisher" -Value "Time Tracker"
            Set-ItemProperty -Path $uninstallKey -Name "DisplayVersion" -Value "1.0.0"
            Set-ItemProperty -Path $uninstallKey -Name "NoModify" -Value 1
            Set-ItemProperty -Path $uninstallKey -Name "NoRepair" -Value 1
            Write-Step "Created uninstall registry entry"
        }
    }

    Write-Header "Installation Complete"
    Write-Host "You can now run '$AppName' from the Start Menu or Desktop." -ForegroundColor Green
}

function Uninstall-Application {
    Write-Header "Uninstalling Application"

    $uninstallKey = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\$AppIdentifier"

    # Check if installed via registry
    if (Test-Path $uninstallKey) {
        $installLocation = Get-ItemPropertyValue -Path $uninstallKey -Name "InstallLocation" -ErrorAction SilentlyContinue
        if ($installLocation) {
            $InstallDir = $installLocation
        }
    }

    # Remove executable and installation directory
    if (Test-Path $InstallDir) {
        Remove-Item -Path $InstallDir -Recurse -Force
        Write-Step "Removed installation directory: $InstallDir"
    }

    # Remove Start Menu shortcut
    $startMenuShortcut = Join-Path $StartMenuFolder "$AppName.lnk"
    if (Test-Path $startMenuShortcut) {
        Remove-Item -Path $startMenuShortcut -Force
        Write-Step "Removed Start Menu shortcut"
    }

    # Remove Desktop shortcut
    if (Test-Path $DesktopShortcut) {
        Remove-Item -Path $DesktopShortcut -Force
        Write-Step "Removed Desktop shortcut"
    }

    # Remove from PATH
    $userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
    if ($userPath -like "*$InstallDir*") {
        $newPath = ($userPath -split ";" | Where-Object { $_ -ne $InstallDir }) -join ";"
        [Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
        Write-Step "Removed installation directory from user PATH"
    }

    # Remove registry entry
    if (Test-Path $uninstallKey) {
        Remove-Item -Path $uninstallKey -Force
        Write-Step "Removed uninstall registry entry"
    }

    # Remove app data (optional - ask user)
    $appDataPath = Join-Path $env:APPDATA "time_tracker_tauri_data.json"
    if (Test-Path $appDataPath) {
        $response = Read-Host "Do you want to remove application data (saved tasks)? [y/N]"
        if ($response -eq "y" -or $response -eq "Y") {
            Remove-Item -Path $appDataPath -Force
            Write-Step "Removed application data"
        }
    }

    Write-Header "Uninstallation Complete"
    Write-Host "$AppName has been removed from your system." -ForegroundColor Green
}

# Main execution
Write-Host ""
Write-Host "Time Tracker - Windows Installation Script" -ForegroundColor Magenta
Write-Host "==========================================" -ForegroundColor Magenta

switch ($Action) {
    "install" {
        Test-Prerequisites
        Build-Application
        Install-Application
    }
    "uninstall" {
        Uninstall-Application
    }
    "build" {
        Test-Prerequisites
        Build-Application
        Write-Header "Build Complete"
        $artifact = Get-BuildArtifactPath
        if ($artifact) {
            Write-Host "Build artifact: $($artifact.Path)" -ForegroundColor Green
        }
    }
}