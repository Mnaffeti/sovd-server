@echo off
REM SOVD Server API Test Script
REM This script tests the SOVD server API endpoints using curl

echo ================================================
echo SOVD Server - API Test Script
echo ================================================
echo.

REM Check if curl is available
curl --version >nul 2>&1
if %errorlevel% neq 0 (
    echo ERROR: curl is not available!
    echo.
    echo Please install curl or use PowerShell's Invoke-RestMethod
    echo Example: Invoke-RestMethod -Uri "http://localhost:8080/health"
    pause
    exit /b 1
)

echo Testing SOVD Server API...
echo Make sure the server is running on http://localhost:8080
echo.

echo ================================================
echo 1. Health Check
echo ================================================
curl -s http://localhost:8080/health | echo.
echo.

echo ================================================
echo 2. Get All Components
echo ================================================
curl -s http://localhost:8080/api/v1/components
echo.
echo.

echo ================================================
echo 3. Get Engine Identification Data
echo ================================================
curl -s "http://localhost:8080/api/v1/components/engine/data?categories=identData"
echo.
echo.

echo ================================================
echo 4. Get VIN from Engine ECU
echo ================================================
curl -s http://localhost:8080/api/v1/components/engine/data/vin
echo.
echo.

echo ================================================
echo 5. Get Engine Software Version
echo ================================================
curl -s http://localhost:8080/api/v1/components/engine/data/swversion
echo.
echo.

echo ================================================
echo 6. Get All Engine Data
echo ================================================
curl -s http://localhost:8080/api/v1/components/engine/data
echo.
echo.

echo ================================================
echo 7. Get Transmission VIN
echo ================================================
curl -s http://localhost:8080/api/v1/components/transmission/data/vin
echo.
echo.

echo ================================================
echo 8. Get Live Data from Engine
echo ================================================
curl -s "http://localhost:8080/api/v1/components/engine/data?categories=liveData"
echo.
echo.

echo ================================================
echo API Test Complete!
echo ================================================

pause