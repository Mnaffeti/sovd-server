# SOVD Server API Test Script (PowerShell)
# This script tests the SOVD server API endpoints using Invoke-RestMethod

Write-Host "================================================" -ForegroundColor Cyan
Write-Host "SOVD Server - API Test Script (PowerShell)" -ForegroundColor Cyan  
Write-Host "================================================" -ForegroundColor Cyan
Write-Host ""

Write-Host "Testing SOVD Server API..." -ForegroundColor Yellow
Write-Host "Make sure the server is running on http://localhost:8080" -ForegroundColor Yellow
Write-Host ""

$baseUrl = "http://localhost:8080"

try {
    Write-Host "================================================" -ForegroundColor Green
    Write-Host "1. Health Check" -ForegroundColor Green
    Write-Host "================================================" -ForegroundColor Green
    $response = Invoke-RestMethod -Uri "$baseUrl/health" -Method Get
    $response | ConvertTo-Json -Depth 3
    Write-Host ""

    Write-Host "================================================" -ForegroundColor Green
    Write-Host "2. Get All Components" -ForegroundColor Green
    Write-Host "================================================" -ForegroundColor Green
    $response = Invoke-RestMethod -Uri "$baseUrl/api/v1/components" -Method Get
    $response | ConvertTo-Json -Depth 3
    Write-Host ""

    Write-Host "================================================" -ForegroundColor Green
    Write-Host "3. Get Engine Identification Data" -ForegroundColor Green
    Write-Host "================================================" -ForegroundColor Green
    $response = Invoke-RestMethod -Uri "$baseUrl/api/v1/components/engine/data?categories=identData" -Method Get
    $response | ConvertTo-Json -Depth 3
    Write-Host ""

    Write-Host "================================================" -ForegroundColor Green
    Write-Host "4. Get VIN from Engine ECU" -ForegroundColor Green
    Write-Host "================================================" -ForegroundColor Green
    $response = Invoke-RestMethod -Uri "$baseUrl/api/v1/components/engine/data/vin" -Method Get
    $response | ConvertTo-Json -Depth 3
    Write-Host ""

    Write-Host "================================================" -ForegroundColor Green
    Write-Host "5. Get Engine Software Version" -ForegroundColor Green
    Write-Host "================================================" -ForegroundColor Green
    $response = Invoke-RestMethod -Uri "$baseUrl/api/v1/components/engine/data/swversion" -Method Get
    $response | ConvertTo-Json -Depth 3
    Write-Host ""

    Write-Host "================================================" -ForegroundColor Green
    Write-Host "6. Get All Engine Data" -ForegroundColor Green
    Write-Host "================================================" -ForegroundColor Green
    $response = Invoke-RestMethod -Uri "$baseUrl/api/v1/components/engine/data" -Method Get
    $response | ConvertTo-Json -Depth 3
    Write-Host ""

    Write-Host "================================================" -ForegroundColor Green
    Write-Host "7. Get Transmission VIN" -ForegroundColor Green
    Write-Host "================================================" -ForegroundColor Green
    $response = Invoke-RestMethod -Uri "$baseUrl/api/v1/components/transmission/data/vin" -Method Get
    $response | ConvertTo-Json -Depth 3
    Write-Host ""

    Write-Host "================================================" -ForegroundColor Green
    Write-Host "8. Get Live Data from Engine" -ForegroundColor Green
    Write-Host "================================================" -ForegroundColor Green
    $response = Invoke-RestMethod -Uri "$baseUrl/api/v1/components/engine/data?categories=liveData" -Method Get
    $response | ConvertTo-Json -Depth 3
    Write-Host ""

    Write-Host "================================================" -ForegroundColor Cyan
    Write-Host "API Test Complete!" -ForegroundColor Cyan
    Write-Host "================================================" -ForegroundColor Cyan

} catch {
    Write-Host "Error testing API: $($_.Exception.Message)" -ForegroundColor Red
    Write-Host "Make sure the SOVD server is running on port 8080" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "Press any key to continue..." -ForegroundColor Gray
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")