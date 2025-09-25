@echo off
REM SOVD Server Build Script
REM This script builds the SOVD server executable

echo ================================================
echo SOVD Server - Build Script
echo ================================================
echo.

REM Check if Go is available
go version >nul 2>&1
if %errorlevel% neq 0 (
    echo ERROR: Go is not installed or not in PATH!
    echo.
    echo Please install Go from: https://golang.org/dl/
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
echo Building SOVD Server for Windows...
go build -o sovd-server.exe main.go
if %errorlevel% neq 0 (
    echo ERROR: Build failed!
    pause
    exit /b 1
)

echo.
echo Building SOVD Server for Linux...
set GOOS=linux
set GOARCH=amd64
go build -o sovd-server-linux main.go
if %errorlevel% neq 0 (
    echo ERROR: Linux build failed!
    pause
    exit /b 1
)

echo.
echo ================================================
echo Build completed successfully!
echo ================================================
echo.
echo Created files:
echo - sovd-server.exe (Windows executable)
echo - sovd-server-linux (Linux executable)
echo.
echo To run the Windows executable:
echo    sovd-server.exe
echo.
echo To run on Linux:
echo    ./sovd-server-linux
echo.

pause