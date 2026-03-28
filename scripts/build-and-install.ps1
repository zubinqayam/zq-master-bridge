# ==============================================================
# ZQ MASTER BRIDGE - FULL BUILD & INSTALL PIPELINE
# PowerShell Master Script
# Author: ZQ AI LOGIC (TM)
#
# USAGE (run from repo root in PowerShell):
#   .\scripts\build-and-install.ps1
#
# Or trigger via VS Code: Terminal > Run Task >
#   "ZQ: FULL PIPELINE (Clone to EXE Install)"
#
# This script handles every step from cloning the repo
# to silently installing the final Windows .exe.
# Each phase is clearly commented so you know exactly
# what is happening at every stage.
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
# If the repo was already cloned (.git folder exists) it pulls
# the latest changes from main instead.
# --------------------------------------------------------------
Write-Host "[PHASE 1] Cloning repository..." -ForegroundColor Yellow
if (-Not (Test-Path ".git")) {
    git clone https://github.com/zubinqayam/zq-master-bridge.git .
    Write-Host "  Cloned successfully." -ForegroundColor Green
} else {
    Write-Host "  Already cloned - pulling latest changes..." -ForegroundColor Gray
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
Write-Host "  [OK] Agent EXE: src-tauri/resources/zq-agent-router.exe" -ForegroundColor Green

# --------------------------------------------------------------
# PHASE 7 - BUILD THE WINDOWS INSTALLER EXE (NSIS)
# This is the main Tauri build step. It:
#   1. Compiles the React 19 + Vite frontend
#   2. Compiles the Rust/Tauri backend (src-tauri/)
#   3. Bundles everything into an NSIS Windows installer
# Output location:
#   src-tauri/target/release/bundle/nsis/*.exe
# NOTE: First run takes 5-15 min (full Rust compile).
#       Subsequent runs are faster (incremental).
# --------------------------------------------------------------
Write-Host "`n[PHASE 7] Building Windows EXE installer (NSIS)..." -ForegroundColor Yellow
Write-Host "  (First run: 5-15 min. Subsequent runs: faster.)" -ForegroundColor Gray
npm run tauri:build -- --bundles nsis
Write-Host "  [OK] Build complete." -ForegroundColor Green

# --------------------------------------------------------------
# PHASE 8 - LOCATE THE BUILT EXE
# Searches the Tauri bundle output directory for the NSIS .exe.
# Prints the full path and file size so you can verify it.
# Exits with error if no EXE is found (means build failed).
# --------------------------------------------------------------
Write-Host "`n[PHASE 8] Locating built installer..." -ForegroundColor Yellow
$exeFiles = Get-ChildItem -Recurse `
    -Path "src-tauri/target/release/bundle/nsis" `
    -Filter "*.exe" `
    -ErrorAction SilentlyContinue

if (-Not $exeFiles) {
    Write-Host "  ERROR: No EXE found. Check Tauri build output above." -ForegroundColor Red
    exit 1
}

$installer = $exeFiles | Select-Object -First 1
$sizeMB    = [math]::Round($installer.Length / 1MB, 2)
Write-Host "  [OK] Installer: $($installer.FullName)" -ForegroundColor Green
Write-Host "       Size:      $sizeMB MB" -ForegroundColor Green

# --------------------------------------------------------------
# PHASE 9 - SILENT INSTALL
# Runs the NSIS installer with these flags:
#   /S           = silent mode (no prompts, no UI)
#   /currentuser = install for current user only (no admin)
# -Wait ensures the script does not exit before install ends.
# After this the app appears in Start Menu > ZQ AI Logic.
# --------------------------------------------------------------
Write-Host "`n[PHASE 9] Installing ZQ Master Bridge silently..." -ForegroundColor Yellow
Start-Process -FilePath $installer.FullName `
    -ArgumentList "/S /currentuser" `
    -Wait
Write-Host "  [OK] Installation complete!" -ForegroundColor Green

# --------------------------------------------------------------
# ALL DONE
# --------------------------------------------------------------
Write-Host "`n========================================" -ForegroundColor Cyan
Write-Host "  ZQ MASTER BRIDGE INSTALLED!" -ForegroundColor Cyan
Write-Host "  Find it in: Start Menu > ZQ AI Logic" -ForegroundColor Cyan
Write-Host "========================================`n" -ForegroundColor Cyan
