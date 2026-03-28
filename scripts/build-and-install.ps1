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
# PHASE 2 - CHECK AND INSTALL PREREQUISITES
# Verifies that Node.js, Rust, and Python are available.
# Auto-installs Rust via rustup if it is missing.
# Node.js and Python must be installed manually if absent.
# --------------------------------------------------------------
Write-Host "`n[PHASE 2] Checking prerequisites..." -ForegroundColor Yellow

# Check Node.js (>= 20 required for Vite + Tauri)
if (-Not (Get-Command node -ErrorAction SilentlyContinue)) {
    Write-Host "  ERROR: Node.js not found." -ForegroundColor Red
    Write-Host "  Install Node.js >= 20 from: https://nodejs.org" -ForegroundColor Red
    exit 1
} else {
    Write-Host "  [OK] Node.js $(node --version)" -ForegroundColor Green
}

# Check Rust (required for Tauri native backend compilation)
# Auto-installs via rustup bootstrap if missing
if (-Not (Get-Command rustup -ErrorAction SilentlyContinue)) {
    Write-Host "  Rust not found - installing rustup..." -ForegroundColor Gray
    Invoke-WebRequest -Uri https://win.rustup.rs -OutFile rustup-init.exe
    .\rustup-init.exe -y --default-toolchain stable
    Remove-Item rustup-init.exe
    Write-Host "  [OK] Rust installed." -ForegroundColor Green
} else {
    Write-Host "  [OK] $(rustc --version)" -ForegroundColor Green
    rustup update stable | Out-Null
}

# Check Python (>= 3.11 for agent sidecar)
if (-Not (Get-Command python -ErrorAction SilentlyContinue)) {
    Write-Host "  ERROR: Python not found." -ForegroundColor Red
    Write-Host "  Install Python >= 3.11 from: https://python.org" -ForegroundColor Red
    exit 1
} else {
    Write-Host "  [OK] $(python --version)" -ForegroundColor Green
}

# --------------------------------------------------------------
# PHASE 3 - INSTALL NODE.JS DEPENDENCIES
# Downloads and installs all JavaScript/TypeScript packages
# defined in package.json:
#   - React 19, TypeScript, Vite (frontend)
#   - @tauri-apps/cli (build tooling)
# --------------------------------------------------------------
Write-Host "`n[PHASE 3] Installing Node.js dependencies..." -ForegroundColor Yellow
npm install
Write-Host "  [OK] Node dependencies installed." -ForegroundColor Green

# --------------------------------------------------------------
# PHASE 4 - INSTALL PYTHON DEPENDENCIES
# Installs the Python packages required by the agent sidecar
# (agents/core/router.py) from agents/requirements.txt.
# --------------------------------------------------------------
Write-Host "`n[PHASE 4] Installing Python agent dependencies..." -ForegroundColor Yellow
pip install -r agents/requirements.txt
Write-Host "  [OK] Python dependencies installed." -ForegroundColor Green

# --------------------------------------------------------------
# PHASE 5 - GENERATE APPLICATION ICONS
# Runs scripts/generate_icons.py which creates:
#   - PNG icons: 32x32, 64x64, 128x128, 256x256
#   - icon.ico:  multi-resolution (16 to 256px)
# These are used by Windows for the EXE, taskbar, Start Menu.
# --------------------------------------------------------------
Write-Host "`n[PHASE 5] Generating application icons..." -ForegroundColor Yellow
python scripts/generate_icons.py
Write-Host "  [OK] Icons saved to src-tauri/icons/" -ForegroundColor Green

# --------------------------------------------------------------
# PHASE 6 - BUNDLE PYTHON AGENT WITH PYINSTALLER
# Compiles agents/core/router.py into a standalone Windows EXE
# (zq-agent-router.exe) placed in src-tauri/resources/.
# This sidecar ships inside the installer so end users do NOT
# need Python installed on their machine.
# PyInstaller is pip-installed automatically if not present.
# --------------------------------------------------------------
Write-Host "`n[PHASE 6] Bundling Python agent (PyInstaller)..." -ForegroundColor Yellow
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
