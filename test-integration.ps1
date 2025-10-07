# Integration Test Script
# Tests the SOVD server with and without the adapter

Write-Host "================================================" -ForegroundColor Cyan
Write-Host "SOVD Integration Test Suite" -ForegroundColor Cyan
Write-Host "================================================" -ForegroundColor Cyan
Write-Host ""

$testResults = @()

function Test-Endpoint {
    param(
        [string]$Name,
        [string]$Url,
        [int]$ExpectedStatus = 200
    )
    
    Write-Host "Testing: $Name..." -NoNewline
    try {
        $response = Invoke-WebRequest -Uri $Url -TimeoutSec 5 -ErrorAction Stop
        if ($response.StatusCode -eq $ExpectedStatus) {
            Write-Host " ✓ PASS" -ForegroundColor Green
            $script:testResults += @{Name=$Name; Status="PASS"}
            return $true
        } else {
            Write-Host " ✗ FAIL (Status: $($response.StatusCode))" -ForegroundColor Red
            $script:testResults += @{Name=$Name; Status="FAIL"}
            return $false
        }
    } catch {
        Write-Host " ✗ FAIL ($($_.Exception.Message))" -ForegroundColor Red
        $script:testResults += @{Name=$Name; Status="FAIL"}
        return $false
    }
}

# Test 1: Check if Go server is running
Write-Host "`n=== Phase 1: Server Availability ===" -ForegroundColor Yellow
Test-Endpoint "Go Server Health" "http://localhost:8080/health"

# Test 2: Check if adapter is running
Write-Host "`n=== Phase 2: Adapter Availability ===" -ForegroundColor Yellow
$adapterAvailable = Test-Endpoint "Rust Adapter Health" "http://localhost:8081/health"

# Test 3: Test SOVD API endpoints
Write-Host "`n=== Phase 3: SOVD API Endpoints ===" -ForegroundColor Yellow
Test-Endpoint "List Components" "http://localhost:8080/api/v1/components"
Test-Endpoint "Get Component Data Items" "http://localhost:8080/api/v1/components/ecu_engine/data"
Test-Endpoint "Read Data Item (VIN)" "http://localhost:8080/api/v1/components/ecu_engine/data/vin"
Test-Endpoint "Read Data Item (Engine Speed)" "http://localhost:8080/api/v1/components/ecu_engine/data/engine_speed"

# Test 4: Verify data source
Write-Host "`n=== Phase 4: Data Source Verification ===" -ForegroundColor Yellow
try {
    $response = Invoke-RestMethod -Uri "http://localhost:8080/api/v1/components/ecu_engine/data/engine_speed" -TimeoutSec 5
    Write-Host "Data Item Response:" -ForegroundColor Cyan
    Write-Host "  Value: $($response.value)" -ForegroundColor White
    Write-Host "  Unit: $($response.unit)" -ForegroundColor White
    Write-Host "  Timestamp: $($response.timestamp)" -ForegroundColor White
    
    if ($adapterAvailable) {
        Write-Host "  Source: UDS Adapter (expected)" -ForegroundColor Green
    } else {
        Write-Host "  Source: Mock Data (expected)" -ForegroundColor Yellow
    }
} catch {
    Write-Host "Failed to retrieve data item: $($_.Exception.Message)" -ForegroundColor Red
}

# Test 5: Test adapter-specific endpoints (if available)
if ($adapterAvailable) {
    Write-Host "`n=== Phase 5: Adapter-Specific Tests ===" -ForegroundColor Yellow
    
    # Test direct adapter call
    try {
        $adapterRequest = @{
            component_id = "ecu_engine"
            data_identifier = "engine_speed"
        } | ConvertTo-Json
        
        $response = Invoke-RestMethod -Uri "http://localhost:8081/api/sovd/read" `
            -Method Post `
            -ContentType "application/json" `
            -Body $adapterRequest `
            -TimeoutSec 5
        
        Write-Host "Direct Adapter Call: ✓ PASS" -ForegroundColor Green
        Write-Host "  Response: $($response | ConvertTo-Json -Compress)" -ForegroundColor White
        $testResults += @{Name="Direct Adapter Call"; Status="PASS"}
    } catch {
        Write-Host "Direct Adapter Call: ✗ FAIL ($($_.Exception.Message))" -ForegroundColor Red
        $testResults += @{Name="Direct Adapter Call"; Status="FAIL"}
    }
} else {
    Write-Host "`n=== Phase 5: Adapter-Specific Tests ===" -ForegroundColor Yellow
    Write-Host "Skipped (adapter not available)" -ForegroundColor Yellow
}

# Summary
Write-Host "`n================================================" -ForegroundColor Cyan
Write-Host "Test Summary" -ForegroundColor Cyan
Write-Host "================================================" -ForegroundColor Cyan

$passCount = ($testResults | Where-Object { $_.Status -eq "PASS" }).Count
$failCount = ($testResults | Where-Object { $_.Status -eq "FAIL" }).Count
$totalCount = $testResults.Count

Write-Host "Total Tests: $totalCount" -ForegroundColor White
Write-Host "Passed: $passCount" -ForegroundColor Green
Write-Host "Failed: $failCount" -ForegroundColor Red

if ($failCount -eq 0) {
    Write-Host "`n✓ All tests passed!" -ForegroundColor Green
    exit 0
} else {
    Write-Host "`n✗ Some tests failed" -ForegroundColor Red
    Write-Host "`nFailed Tests:" -ForegroundColor Yellow
    $testResults | Where-Object { $_.Status -eq "FAIL" } | ForEach-Object {
        Write-Host "  - $($_.Name)" -ForegroundColor Red
    }
    exit 1
}
