# SOVD Server

A Service Oriented Vehicle Diagnostics (SOVD) server implementation in Go, providing HTTP/REST APIs for vehicle component diagnostics and data retrieval. **Now integrated with a Rust-based SOVD2UDS adapter for real UDS protocol communication!**

## Overview

This SOVD server implements the SOVD specification with a focus on vehicle diagnostics. It provides:
- **Mock data mode** for development and testing without vehicle hardware
- **Real UDS communication** via integrated Rust SOVD2UDS adapter
- **Automatic fallback** from UDS to mock data when adapter unavailable
- **RESTful API** following SOVD specification for easy integration

## Features

- **RESTful API** following SOVD specification
- **OpenAPI 3.0 specification** for API documentation
- **Dual-mode operation**: Real UDS communication OR mock data
- **SOVD2UDS Adapter Integration**: Rust-based UDS protocol bridge
- **Component discovery** - list all available vehicle components
- **Data item discovery** - list available data items per component with category filtering
- **Data retrieval** - get specific data item values (e.g., VIN, software version)
- **Structured for UDS integration** - ready to integrate with C++ UDS library
- **CORS support** for web applications

## Architecture

```
┌─────────────────┐         ┌──────────────────┐         ┌─────────────────┐
│   SOVD Client   │  HTTP   │   Go SOVD Server │  HTTP   │  Rust Adapter   │
│  (REST API)     │ ──────> │   (Port 8080)    │ ──────> │  (Port 8081)    │
└─────────────────┘         └──────────────────┘         └─────────────────┘
                                      │                            │
                                      │ Fallback                   │ FFI
                                      ▼                            ▼
                                 ┌─────────┐              ┌──────────────┐
                                 │  Mock   │              │ UDS/DoIP     │
                                 │  Data   │              │ C Libraries  │
                                 └─────────┘              └──────────────┘
                                                                  │
                                                                  ▼
                                                           ┌──────────────┐
                                                           │ Vehicle ECUs │
                                                           └──────────────┘
```

## Project Structure

```
sovd-server/
├── main.go                          # Application entry point with adapter integration
├── go.mod                           # Go module definition
├── openapi.yaml                     # OpenAPI 3.0 specification
├── INTEGRATION.md                   # Detailed integration guide
├── start-integrated.ps1             # PowerShell script to start both services
├── start-integrated.bat             # Batch script to start both services
├── internal/
│   ├── handlers/
│   │   └── sovd_handler.go         # HTTP request handlers
│   ├── models/
│   │   └── models.go               # Data models and structs
│   └── services/
│       └── sovd_service.go         # Business logic with adapter integration
├── pkg/
│   └── uds/
│       └── adapter_client.go       # HTTP client for Rust adapter
└── sovd2uds-adapter/                # Rust SOVD2UDS adapter (separate project)
    ├── src/
    ├── Cargo.toml
    └── README.md
```

## Quick Start

### Option 1: With SOVD2UDS Adapter (Real UDS Communication)

**Prerequisites:**
- Go 1.21 or later
- Rust 1.70 or later (for building the adapter)
- UDS/DoIP C libraries (optional, for real vehicle communication)

**Steps:**

1. **Build the Rust adapter:**
   ```powershell
   cd sovd2uds-adapter
   cargo build --release
   cd ..
   ```

2. **Start both services:**
   ```powershell
   .\start-integrated.ps1
   ```

   This script will:
   - Start the Rust adapter on port 8081
   - Wait for it to be healthy
   - Start the Go server on port 8080
   - Handle cleanup when you press Ctrl+C

3. **Verify the system is running:**
   ```powershell
   curl http://localhost:8080/health
   curl http://localhost:8081/health
   ```

4. **Test UDS communication:**
   ```powershell
   curl http://localhost:8080/api/v1/components/ecu_engine/data/engine_speed
   ```

### Option 2: Mock Data Only (Development Mode)

**Prerequisites:**
- Go 1.21 or later

**Steps:**

1. **Download dependencies:**
   ```powershell
   go mod tidy
   ```

2. **Run the server:**
   ```powershell
   go run main.go
   ```
   
   The server will detect that the adapter is unavailable and automatically use mock data.

3. **The server will start on port 8080:**
   ```
   Using SOVD2UDS adapter at: http://localhost:8081
   WARN: SOVD2UDS adapter unavailable, using mock data
   Starting SOVD Server on :8080
   ```

### Configuration

**Environment Variables:**
- `SOVD_ADAPTER_URL` - URL of the Rust adapter (default: `http://localhost:8081`)
- `PORT` - Server port (default: `8080`)
- `GIN_MODE` - Gin framework mode (`release` or `debug`)

**Example:**
```powershell
$env:SOVD_ADAPTER_URL = "http://localhost:8081"
$env:GIN_MODE = "release"
go run main.go
```

## Prerequisites

- Go 1.21 or later
### Installation and Running

1. **Clone/Navigate to the project directory:**
   ```powershell
   cd "c:\Users\XXXXX\Desktop\SOVD"
   ```

2. **Download dependencies:**
   ```powershell
   go mod tidy
   ```

3. **Run the server:**
   ```powershell
   go run main.go
   ```

4. **The server will start on port 8080:**
   ```
   Starting SOVD Server on :8080
   API Documentation: http://localhost:8080/
   Health Check: http://localhost:8080/health
   Example VIN Request: http://localhost:8080/api/v1/components/engine/data/vin
   ```

## API Usage Examples

### 1. Health Check
```bash
curl http://localhost:8080/health
```

### 2. Get All Components
```bash
curl http://localhost:8080/api/v1/components
```

### 3. Get Engine Component Identification Data (VIN Example)
```bash
curl "http://localhost:8080/api/v1/components/engine/data?categories=identData"
```

**Response:**
```json
{
  "items": [
    {
      "id": "vin",
      "name": "Vehicle Identification Number",
      "category": "identData",
      "dataType": "string",
      "description": "Unique vehicle identification number"
    },
    {
      "id": "swversion",
      "name": "Software Version",
      "category": "identData",
      "dataType": "string",
      "description": "ECU software version"
    }
  ]
}
```

### 4. Read VIN from Engine ECU
```bash
curl http://localhost:8080/api/v1/components/engine/data/vin
```

**Response:**
```json
{
  "id": "vin",
  "name": "Vehicle Identification Number",
  "category": "identData",
  "data": "V3CT0RV3H1CL3123",
  "timestamp": "2024-09-25T10:30:00Z",
  "quality": "good"
}
```

### 5. Get All Engine Data (no category filter)
```bash
curl http://localhost:8080/api/v1/components/engine/data
```

### 6. Get Live Data Only
```bash
curl "http://localhost:8080/api/v1/components/engine/data?categories=liveData"
```

## Available Components and Data

### Components
- **engine** - Engine Control Unit
- **transmission** - Transmission Control Unit  
- **abs** - Anti-lock Braking System
- **airbag** - Airbag Control Unit
- **bcm** - Body Control Module

### Data Categories
- **identData** - Identification data (VIN, software version, hardware version, serial number)
- **liveData** - Live sensor data (RPM, temperature, wheel speed, etc.)

### Sample Data Items

#### Engine ECU
- **vin**: "V3CT0RV3H1CL3123"
- **swversion**: "ECU_V2.1.4_BUILD_20241001"
- **hwversion**: "HW_V1.0.2"
- **serialnumber**: "ECU123456789ABC"
- **enginerpm**: 850.5
- **coolanttemp**: 89.2

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/health` | Health check |
| GET | `/api/v1/components` | Get all components |
| GET | `/api/v1/components/{component_id}/data` | Get component data items |
| GET | `/api/v1/components/{component_id}/data?categories={categories}` | Get filtered data items |
| GET | `/api/v1/components/{component_id}/data/{data_id}` | Get specific data item value |

## Future UDS Integration

The project is structured to easily integrate with your  UDS library:

### Integration Points

1. **UDS Client Structure** - `pkg/uds/uds_client.go` contains placeholder structures
2. **Service Layer** - `internal/services/sovd_service.go` contains mock data that can be replaced with UDS calls
3. **CGO Integration** - Ready for C++ library binding

### Integration Steps

1. **Create CGO bindings** for your  UDS library
2. **Replace mock data** in `sovd_service.go` with actual UDS requests
3. **Update UDS client** implementation in `pkg/uds/uds_client.go`
4. **Add error handling** for UDS communication failures
5. **Implement connection management** for CAN/Ethernet interfaces

### Example UDS Integration

```go
// In sovd_service.go, replace mock data with:
func (s *SOVDService) GetDataItemValue(componentID, dataID string) (*models.DataItemValue, error) {
    if dataID == "vin" {
        // Use UDS to read VIN (DID 0xF190)
        response, err := s.udsClient.ReadDataByIdentifier(uds.DID_VIN)
        if err != nil {
            return nil, err
        }
        
        return &models.DataItemValue{
            ID:       "vin",
            Name:     "Vehicle Identification Number",
            Category: "identData",
            Data:     string(response.Data),
            Quality:  models.QualityGood,
        }, nil
    }
    // ... handle other data items
}
```

## Building for Production

### Build executable:
```powershell
go build -o sovd-server.exe main.go
```

### Cross-compile for Linux:
```powershell
$env:GOOS="linux"; $env:GOARCH="amd64"; go build -o sovd-server main.go
```

## Testing

### Manual Testing
Use the provided curl examples or any HTTP client (Postman)


## Configuration

The server currently runs on port 8080. To change the port, modify the `router.Run(":8080")` line in `main.go`.



## Support

For questions or issues:
- Create GitHub issues for bugs/feature requests
- Check the OpenAPI specification at `/openapi.yaml`
- Use the health check endpoint to verify server status
