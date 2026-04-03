# Canonical PyInstaller entrypoint for the Python sidecar.
# Uses the tracked zq-agent-router.spec so local and CI builds stay aligned.

[CmdletBinding()]
param(
    [switch]$InstallDependencies
)

$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $PSScriptRoot
$distPath = Join-Path $repoRoot "src-tauri\resources"
$workPath = Join-Path $repoRoot "build\pyinstaller"
$specPath = Join-Path $repoRoot "zq-agent-router.spec"
$outputFile = Join-Path $distPath "zq-agent-router.exe"

function Require-Command([string]$Name) {
    if (-not (Get-Command $Name -ErrorAction SilentlyContinue)) {
        throw "Required command not found: $Name"
    }
}

Require-Command "python"

Push-Location $repoRoot
try {
    if (-not (Test-Path $specPath)) {
        throw "PyInstaller spec not found: $specPath"
    }

    New-Item -ItemType Directory -Force -Path $distPath | Out-Null
    New-Item -ItemType Directory -Force -Path $workPath | Out-Null

    if ($InstallDependencies) {
        python -m pip install --upgrade pip
        python -m pip install pyinstaller

        if (Test-Path "agents\requirements.txt") {
            python -m pip install -r "agents\requirements.txt"
        }
    } else {
        $pyInstallerReady = $true
        try {
            python -m PyInstaller --version | Out-Null
        } catch {
            $pyInstallerReady = $false
        }

        if (-not $pyInstallerReady) {
            python -m pip install pyinstaller
        }
    }

    if (Test-Path $outputFile) {
        Remove-Item -LiteralPath $outputFile -Force
    }

    python -m PyInstaller `
        --noconfirm `
        --clean `
        --distpath $distPath `
        --workpath $workPath `
        $specPath

    if (-not (Test-Path $outputFile)) {
        throw "Expected sidecar output was not created: $outputFile"
    }

    Write-Host "Built sidecar: $outputFile" -ForegroundColor Green
} finally {
    Pop-Location
}
