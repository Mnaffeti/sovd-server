# Integrated SOVD Server Startup Script
# Starts both the Rust adapter and Go server with proper process management

Write-Host "================================================" -ForegroundColor Cyan
Write-Host "Starting SOVD2UDS Integrated System" -ForegroundColor Cyan
Write-Host "================================================" -ForegroundColor Cyan
Write-Host ""

# Check if Rust adapter is built
$adapterExe = if (Test-Path "sovd2uds-adapter\target\release\sovd2uds_adapter.exe") {
    "sovd2uds-adapter\target\release\sovd2uds_adapter.exe"
} elseif (Test-Path "sovd2uds-adapter\target\debug\sovd2uds_adapter.exe") {
    "sovd2uds-adapter\target\debug\sovd2uds_adapter.exe"
} else {
    Write-Host "ERROR: Rust adapter not built!" -ForegroundColor Red
    Write-Host "Please build it first with: cd sovd2uds-adapter && cargo build --release" -ForegroundColor Yellow
    exit 1
}

# Check if Go server is built
if (-not (Test-Path "sovd-server.exe")) {
    Write-Host "Building Go server..." -ForegroundColor Yellow
    $buildResult = go build -o sovd-server.exe .
    if ($LASTEXITCODE -ne 0) {
        Write-Host "ERROR: Failed to build Go server" -ForegroundColor Red
        exit 1
    }
}

# Start Rust adapter in background
Write-Host "Starting Rust SOVD2UDS Adapter on port 8081..." -ForegroundColor Green
$adapterProcess = Start-Process -FilePath $adapterExe -PassThru -WindowStyle Normal

# Wait for adapter to start
Start-Sleep -Seconds 2

# Check if adapter is running
if ($adapterProcess.HasExited) {
    Write-Host "ERROR: Adapter failed to start" -ForegroundColor Red
    exit 1
}

# Test adapter health
try {
    $response = Invoke-WebRequest -Uri "http://localhost:8081/health" -TimeoutSec 5 -ErrorAction Stop
    Write-Host "✓ Adapter is healthy" -ForegroundColor Green
} catch {
    Write-Host "WARNING: Adapter health check failed, but continuing..." -ForegroundColor Yellow
}

Write-Host ""
Write-Host "Starting Go SOVD Server on port 8080..." -ForegroundColor Green
Write-Host ""
Write-Host "================================================" -ForegroundColor Cyan
Write-Host "System Running:" -ForegroundColor Cyan
Write-Host "  - SOVD2UDS Adapter: http://localhost:8081" -ForegroundColor White
Write-Host "  - SOVD Server:      http://localhost:8080" -ForegroundColor White
Write-Host "  - Health Check:     http://localhost:8080/health" -ForegroundColor White
Write-Host "================================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Press Ctrl+C to stop both services" -ForegroundColor Yellow
Write-Host ""

# Set environment variable for adapter URL
$env:SOVD_ADAPTER_URL = "http://localhost:8081"

# Register cleanup handler
try {
    # Start Go server (this will block)
    & .\sovd-server.exe
} finally {
    # Cleanup: Stop adapter when Go server exits
    Write-Host ""
    Write-Host "Stopping SOVD2UDS Adapter..." -ForegroundColor Yellow
    if (-not $adapterProcess.HasExited) {
        Stop-Process -Id $adapterProcess.Id -Force
        Write-Host "✓ Adapter stopped" -ForegroundColor Green
    }
    Write-Host "Shutdown complete" -ForegroundColor Green
}
