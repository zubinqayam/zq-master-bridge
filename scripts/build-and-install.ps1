# ==============================================================
# ZQ MASTER BRIDGE - FULL BUILD & INSTALL PIPELINE
# PowerShell Master Script
# Author: ZQ AI LOGIC (TM)
#
# USAGE (run from repo root in PowerShell):
#   .\scripts\build-and-install.ps1
#
# Or from VS Code: Terminal > Run Task > "ZQ: FULL PIPELINE"
#
# This script covers every step from cloning the repo all the
# way to silently installing the final Windows .exe on the
# current machine. Each phase is clearly commented.
# ==============================================================

# Stop immediately on any unhandled error
$ErrorActionPreference = "Stop"

Write-Host "`n========================================" -ForegroundColor Cyan
Write-Host "  ZQ MASTER BRIDGE - BUILD PIPELINE" -ForegroundColor Cyan
Write-Host "========================================`n" -ForegroundColor Cyan

# --------------------------------------------------------------
# PHASE 1 - CLONE THE REPOSITORY
# Clones zq-master-bridge from GitHub into the current directory.
# If the repo was already cloned (i.e. .git folder exists),
# this phase is safely skipped.
# --------------------------------------------------------------
Write-Host "[PHASE 1] Cloning repository..." -ForegroundColor Yellow
if (-Not (Test-Path ".git")) {
    git clone https://github.com/zubinqayam/zq-master-bridge.git .
    Write-Host "  Cloned successfully." -ForegroundColor Green
} else {
    Write-Host "  Already cloned. Pulling latest changes..." -ForegroundColor Gray
    git pull origin main
}

# --------------------------------------------------------------
# PHASE 2 - CHECK & INSTALL PREREQUISITES
# Verifies Node.js, Rust, Python, and pip are installed.
# Installs rustup if Rust is missing.
# --------------------------------------------------------------
Write-Host "`n[PHASE 2] Checking prerequisites..." -ForegroundColor Yellow

# Node.js (>= 20 required for Vite + Tauri)
if (-Not (Get-Command node -ErrorAction SilentlyContinue)) {
    Write-Host "  ERROR: Node.js not found. Install from https://nodejs.org" -ForegroundColor Red
    exit 1
} else {
    Write-Host "  Node.js: $(node --version)" -ForegroundColor Green
}

# Rust (stable, required for Tauri native backend)
if (-Not (Get-Command rustup -ErrorAction SilentlyContinue)) {
    Write-Host "  Rust not found. Installing rustup..." -ForegroundColor Gray
    Invoke-WebRequest -Uri https://win.rustup.rs -OutFile rustup-init.exe
    .\rustup-init.exe -y --default-toolchain stable
    Remove-Item rustup-init.exe
} else {
    Write-Host "  Rust: $(rustc --version)" -ForegroundColor Green
    rustup update stable | Out-Null
}

# Python (>= 3.11 required for agent sidecar)
if (-Not (Get-Command python -ErrorAction SilentlyContinue)) {
    Write-Host "  ERROR: Python not found. Install from https://python.org" -ForegroundColor Red
    exit 1
} else {
    Write-Host "  Python: $(python --version)" -ForegroundColor Green
}

# --------------------------------------------------------------
# PHASE 3 - INSTALL NODE DEPENDENCIES
# Downloads and installs all JavaScript/TypeScript packages
# listed in package.json (React 19, Vite, Tauri CLI, etc.)
# --------------------------------------------------------------
Write-Host "`n[PHASE 3] Installing Node.js dependencies (npm install)..." -ForegroundColor Yellow
npm install
Write-Host "  Node dependencies installed." -ForegroundColor Green

# --------------------------------------------------------------
# PHASE 4 - INSTALL PYTHON DEPENDENCIES
# Installs packages needed by the Python agent sidecar
# (agents/core/router.py) using pip.
# --------------------------------------------------------------
Write-Host "`n[PHASE 4] Installing Python dependencies..." -ForegroundColor Yellow
pip install -r agents/requirements.txt
Write-Host "  Python dependencies installed." -ForegroundColor Green

# --------------------------------------------------------------
# PHASE 5 - GENERATE APP ICONS
# Runs the icon generator script to produce all required PNG
# sizes plus a multi-resolution icon.ico (16x16 to 256x256)
# that Windows uses for the EXE, installer, and Start Menu.
# --------------------------------------------------------------
Write-Host "`n[PHASE 5] Generating application icons..." -ForegroundColor Yellow
python scripts/generate_icons.py
Write-Host "  Icons generated in src-tauri/icons/" -ForegroundColor Green

# --------------------------------------------------------------
# PHASE 6 - BUNDLE PYTHON AGENT WITH PYINSTALLER
# Packages agents/core/router.py as a self-contained .exe
# sidecar (zq-agent-router.exe) into src-tauri/resources/.
# This means the end user does NOT need Python installed.
# PyInstaller is installed automatically if not present.
# --------------------------------------------------------------
Write-Host "`n[PHASE 6] Bundling Python agent with PyInstaller..." -ForegroundColor Yellow
pip install pyinstaller | Out-Null
pyinstaller `
    --onefile `
    --distpath src-tauri/resources `
    --name zq-agent-router `
    --clean `
    agents/core/router.py
Write-Host "  Python agent bundled to src-tauri/resources/zq-agent-router.exe" -ForegroundColor Green

# --------------------------------------------------------------
# PHASE 7 - BUILD WINDOWS INSTALLER (.EXE)
# Compiles the full app:
#   - React 19 UI (Vite)
#   - Rust/Tauri native backend
#   - NSIS installer wrapper
# Output: src-tauri/target/release/bundle/nsis/*.exe
# This step takes the longest (5-15 min on first run).
# --------------------------------------------------------------
Write-Host "`n[PHASE 7] Building Windows EXE installer (NSIS)..." -ForegroundColor Yellow
Write-Host "  This may take 5-15 minutes on first run (Rust compile)." -ForegroundColor Gray
npm run tauri:build -- --bundles nsis
Write-Host "  Build complete." -ForegroundColor Green

# --------------------------------------------------------------
# PHASE 8 - LOCATE THE BUILT EXE
# Searches for the NSIS installer output in the bundle folder.
# Prints its path and file size for confirmation.
# --------------------------------------------------------------
Write-Host "`n[PHASE 8] Locating built installer..." -ForegroundColor Yellow
$exeFiles = Get-ChildItem -Recurse -Path "src-tauri/target/release/bundle/nsis" -Filter "*.exe" -ErrorAction SilentlyContinue
if (-Not $exeFiles) {
    Write-Host "  ERROR: No EXE found in bundle output. Check build logs." -ForegroundColor Red
    exit 1
}
$installer = $exeFiles | Select-Object -First 1
$sizeMB = [math]::Round($installer.Length / 1MB, 2)
Write-Host "  Found: $($installer.FullName)" -ForegroundColor Green
Write-Host "  Size:  $sizeMB MB" -ForegroundColor Green

# --------------------------------------------------------------
# PHASE 9 - INSTALL THE EXE SILENTLY
# Runs the NSIS installer with:
#   /S           = silent mode (no GUI, no clicks needed)
#   /currentuser = installs for current user only (no admin)
# Waits for installation to finish before continuing.
# --------------------------------------------------------------
Write-Host "`n[PHASE 9] Installing ZQ Master Bridge silently..." -ForegroundColor Yellow
Start-Process -FilePath $installer.FullName -ArgumentList "/S /currentuser" -Wait
Write-Host "  Installation complete!" -ForegroundColor Green

# --------------------------------------------------------------
# DONE
# --------------------------------------------------------------
Write-Host "`n========================================" -ForegroundColor Cyan
Write-Host "  ZQ MASTER BRIDGE INSTALLED!" -ForegroundColor Cyan
Write-Host "  Check Start Menu > ZQ AI Logic" -ForegroundColor Cyan
Write-Host "========================================`n" -ForegroundColor Cyan
