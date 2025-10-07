@echo off
REM Integrated SOVD Server Startup Script
REM Starts both the Rust adapter and Go server

echo ================================================
echo Starting SOVD2UDS Integrated System
echo ================================================
echo.

REM Check if Rust adapter is built
if not exist "sovd2uds-adapter\target\release\sovd2uds_adapter.exe" (
    if not exist "sovd2uds-adapter\target\debug\sovd2uds_adapter.exe" (
        echo ERROR: Rust adapter not built!
        echo Please build it first with: cd sovd2uds-adapter ^&^& cargo build --release
        pause
        exit /b 1
    )
)

REM Check if Go server is built
if not exist "sovd-server.exe" (
    echo Building Go server...
    go build -o sovd-server.exe .
    if errorlevel 1 (
        echo ERROR: Failed to build Go server
        pause
        exit /b 1
    )
)

REM Start Rust adapter in background
echo Starting Rust SOVD2UDS Adapter on port 8081...
cd sovd2uds-adapter
if exist "target\release\sovd2uds_adapter.exe" (
    start "SOVD2UDS Adapter" target\release\sovd2uds_adapter.exe
) else (
    start "SOVD2UDS Adapter" target\debug\sovd2uds_adapter.exe
)
cd ..

REM Wait a moment for adapter to start
timeout /t 2 /nobreak >nul

REM Start Go server
echo Starting Go SOVD Server on port 8080...
echo.
echo ================================================
echo System Running:
echo   - SOVD2UDS Adapter: http://localhost:8081
echo   - SOVD Server:      http://localhost:8080
echo   - Health Check:     http://localhost:8080/health
echo ================================================
echo.
echo Press Ctrl+C to stop the Go server
echo (Note: You'll need to manually close the adapter window)
echo.

set SOVD_ADAPTER_URL=http://localhost:8081
sovd-server.exe
