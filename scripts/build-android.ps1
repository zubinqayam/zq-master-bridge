# Local Android bootstrap/build helper for Tauri 2.
# This supports developer APK builds; Windows remains the official packaged release target.

[CmdletBinding()]
param(
    [switch]$InitOnly,
    [switch]$Release
)

$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $PSScriptRoot
$defaultSdkRoot = Join-Path $env:LOCALAPPDATA "Android\Sdk"

function Require-Command([string]$Name) {
    if (-not (Get-Command $Name -ErrorAction SilentlyContinue)) {
        throw "Required command not found: $Name"
    }
}

function Ensure-EnvPath([string]$PathToAdd) {
    if (-not $PathToAdd) {
        return
    }

    $current = @($env:Path -split ';')
    if ($current -notcontains $PathToAdd) {
        $env:Path = "$PathToAdd;$env:Path"
    }
}

Require-Command "node"
Require-Command "python"
Require-Command "rustup"
Require-Command "java"

if (-not $env:ANDROID_HOME -and (Test-Path $defaultSdkRoot)) {
    $env:ANDROID_HOME = $defaultSdkRoot
}

if (-not $env:ANDROID_SDK_ROOT -and $env:ANDROID_HOME) {
    $env:ANDROID_SDK_ROOT = $env:ANDROID_HOME
}

if (-not $env:ANDROID_HOME -or -not (Test-Path $env:ANDROID_HOME)) {
    throw "ANDROID_HOME is not configured and the default SDK path was not found."
}

Ensure-EnvPath (Join-Path $env:ANDROID_HOME "platform-tools")
Ensure-EnvPath (Join-Path $env:ANDROID_HOME "cmdline-tools\latest\bin")

if (-not $env:JAVA_HOME) {
    $javaCommand = Get-Command java
    $javaBin = Split-Path -Parent $javaCommand.Source
    $env:JAVA_HOME = Split-Path -Parent $javaBin
}

$targets = @("aarch64-linux-android", "armv7-linux-androideabi", "x86_64-linux-android", "i686-linux-android")
foreach ($target in $targets) {
    $isInstalled = rustup target list --installed | Select-String -SimpleMatch $target
    if (-not $isInstalled) {
        rustup target add $target
    }
}

Push-Location $repoRoot
try {
    npx tauri android init --ci

    if ($InitOnly) {
        Write-Host "Android target initialized." -ForegroundColor Green
        return
    }

    if ($Release) {
        npx tauri android build --apk --target aarch64 x86_64
    } else {
        npx tauri android build --debug --apk --target aarch64 x86_64
    }
} finally {
    Pop-Location
}
