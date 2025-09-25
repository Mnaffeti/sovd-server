@echo off
REM SOVD Server Extended API Test Script
REM This script tests all SOVD server API endpoints including new service types

echo ================================================
echo SOVD Server - Extended API Test Script
echo ================================================
echo.

REM Check if curl is available
curl --version >nul 2>&1
if %errorlevel% neq 0 (
    echo ERROR: curl is not available!
    echo.
    echo Please install curl or use PowerShell's Invoke-RestMethod
    pause
    exit /b 1
)

echo Testing SOVD Server Extended API...
echo Make sure the server is running on http://localhost:8080
echo.

echo ================================================
echo 1. Health Check
echo ================================================
curl -s http://localhost:8080/health
echo.
echo.

echo ================================================
echo 2. Get All Components
echo ================================================
curl -s http://localhost:8080/api/v1/components
echo.
echo.

echo ================================================
echo 3. Get Engine Diagnostic Data (New Category)
echo ================================================
curl -s "http://localhost:8080/api/v1/components/engine/data?categories=diagnosticData"
echo.
echo.

echo ================================================
echo 4. Get Engine DTC Count
echo ================================================
curl -s http://localhost:8080/api/v1/components/engine/data/dtc_count
echo.
echo.

echo ================================================
echo 5. Get Engine DTC List
echo ================================================
curl -s http://localhost:8080/api/v1/components/engine/data/dtc_list
echo.
echo.

echo ================================================
echo 6. Get Engine Configuration Data
echo ================================================
curl -s http://localhost:8080/api/v1/components/engine/data/ecu_config
echo.
echo.

echo ================================================
echo 7. Control Engine Actuator - Start Fuel Pump
echo ================================================
curl -s -X POST http://localhost:8080/api/v1/components/engine/actuators/control ^
  -H "Content-Type: application/json" ^
  -d "{\"actuator_id\":\"fuel_pump\",\"action\":\"start\"}"
echo.
echo.

echo ================================================
echo 8. Control Engine Actuator - Set Throttle Value
echo ================================================
curl -s -X POST http://localhost:8080/api/v1/components/engine/actuators/control ^
  -H "Content-Type: application/json" ^
  -d "{\"actuator_id\":\"throttle\",\"action\":\"set_value\",\"value\":45.5}"
echo.
echo.

echo ================================================
echo 9. Control BCM Actuator - Turn On Headlights
echo ================================================
curl -s -X POST http://localhost:8080/api/v1/components/bcm/actuators/control ^
  -H "Content-Type: application/json" ^
  -d "{\"actuator_id\":\"headlights\",\"action\":\"start\",\"duration\":300}"
echo.
echo.

echo ================================================
echo 10. Read DTCs from Engine
echo ================================================
curl -s -X POST http://localhost:8080/api/v1/components/engine/dtcs ^
  -H "Content-Type: application/json" ^
  -d "{\"action\":\"read\"}"
echo.
echo.

echo ================================================
echo 11. Clear All DTCs from Engine
echo ================================================
curl -s -X POST http://localhost:8080/api/v1/components/engine/dtcs ^
  -H "Content-Type: application/json" ^
  -d "{\"action\":\"clear\"}"
echo.
echo.

echo ================================================
echo 12. Clear Specific DTCs from Engine
echo ================================================
curl -s -X POST http://localhost:8080/api/v1/components/engine/dtcs ^
  -H "Content-Type: application/json" ^
  -d "{\"action\":\"clear\",\"dtcs\":[\"P0171\"]}"
echo.
echo.

echo ================================================
echo 13. Get Freeze Frame Data
echo ================================================
curl -s -X POST http://localhost:8080/api/v1/components/engine/dtcs ^
  -H "Content-Type: application/json" ^
  -d "{\"action\":\"freeze_frame\"}"
echo.
echo.

echo ================================================
echo 14. Execute Diagnostic Routine
echo ================================================
curl -s -X POST http://localhost:8080/api/v1/components/engine/services ^
  -H "Content-Type: application/json" ^
  -d "{\"service_type\":\"routine\",\"parameters\":{\"routine_id\":\"engine_compression_test\",\"timeout\":30000}}"
echo.
echo.

echo ================================================
echo 15. Security Access Service
echo ================================================
curl -s -X POST http://localhost:8080/api/v1/components/engine/services ^
  -H "Content-Type: application/json" ^
  -d "{\"service_type\":\"security_access\",\"parameters\":{\"security_level\":2}}"
echo.
echo.

echo ================================================
echo 16. Session Control Service
echo ================================================
curl -s -X POST http://localhost:8080/api/v1/components/engine/services ^
  -H "Content-Type: application/json" ^
  -d "{\"service_type\":\"session_control\",\"parameters\":{\"session_type\":\"extended_diagnostic\"}}"
echo.
echo.

echo ================================================
echo 17. Test Invalid Actuator (Error Case)
echo ================================================
curl -s -X POST http://localhost:8080/api/v1/components/engine/actuators/control ^
  -H "Content-Type: application/json" ^
  -d "{\"actuator_id\":\"invalid_actuator\",\"action\":\"start\"}"
echo.
echo.

echo ================================================
echo 18. Test Invalid Component (Error Case)
echo ================================================
curl -s -X POST http://localhost:8080/api/v1/components/invalid_component/dtcs ^
  -H "Content-Type: application/json" ^
  -d "{\"action\":\"read\"}"
echo.
echo.

echo ================================================
echo Extended API Test Complete!
echo ================================================
echo.
echo Summary of new features tested:
echo - New data categories: diagnosticData, configData
echo - Actuator control with different actions
echo - DTC management (read, clear, freeze frame)
echo - Generic service execution (routines, security, sessions)
echo - Error handling for invalid requests
echo.

pause