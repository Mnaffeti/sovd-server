@echo off
REM SOVD Server Setup and Run Script
REM This script helps set up and run the SOVD server

echo ================================================
echo SOVD Server - Setup and Run Script
echo ================================================
echo.

REM Check if Go is available
go version >nul 2>&1
if %errorlevel% neq 0 (
    echo ERROR: Go is not installed or not in PATH!
    echo.
    echo Please install Go from: https://golang.org/dl/
    echo Make sure to add Go to your PATH environment variable.
    echo.
    echo After installing Go, run this script again.
    pause
    exit /b 1
)

echo Go is available:
go version
echo.

echo Downloading dependencies...
go mod tidy
if %errorlevel% neq 0 (
    echo ERROR: Failed to download dependencies!
    pause
    exit /b 1
)

echo.
echo Dependencies downloaded successfully!
echo.

echo Starting SOVD Server...
echo The server will start on http://localhost:8080
echo.
echo Available endpoints:
echo - Health Check: http://localhost:8080/health
echo - All Components: http://localhost:8080/api/v1/components
echo - Engine VIN: http://localhost:8080/api/v1/components/engine/data/vin
echo - Engine Ident Data: http://localhost:8080/api/v1/components/engine/data?categories=identData
echo.
echo Press Ctrl+C to stop the server
echo.

go run main.go

pause