# Build script for SOVD2UDS Adapter (Windows PowerShell)

param(
    [string]$BuildType = "release"
)

Write-Host "Building SOVD2UDS Adapter..." -ForegroundColor Cyan

# Check if Rust is installed
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "Error: Rust is not installed. Please install Rust from https://rustup.rs/" -ForegroundColor Red
    exit 1
}

# Check environment variables
if (-not $env:UDS_INCLUDE_PATH) {
    Write-Host "Warning: UDS_INCLUDE_PATH not set. Using default: ..\libudsclient\include" -ForegroundColor Yellow
    $env:UDS_INCLUDE_PATH = "..\libudsclient\include"
}

if (-not $env:DOIP_INCLUDE_PATH) {
    Write-Host "Warning: DOIP_INCLUDE_PATH not set. Using default: ..\libdoipclient\include" -ForegroundColor Yellow
    $env:DOIP_INCLUDE_PATH = "..\libdoipclient\include"
}

Write-Host "UDS_INCLUDE_PATH: $env:UDS_INCLUDE_PATH"
Write-Host "DOIP_INCLUDE_PATH: $env:DOIP_INCLUDE_PATH"

# Build
if ($BuildType -eq "debug") {
    Write-Host "Building in debug mode..." -ForegroundColor Cyan
    cargo build
    Write-Host "Build complete! Binary: target\debug\sovd2uds-adapter.exe" -ForegroundColor Green
} else {
    Write-Host "Building in release mode..." -ForegroundColor Cyan
    cargo build --release
    Write-Host "Build complete! Binary: target\release\sovd2uds-adapter.exe" -ForegroundColor Green
}

# Run tests
Write-Host ""
Write-Host "Running tests..." -ForegroundColor Cyan
cargo test

Write-Host ""
Write-Host "Build successful!" -ForegroundColor Green
