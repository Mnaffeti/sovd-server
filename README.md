# SOVD Server

A Service Oriented Vehicle Diagnostics (SOVD) server implementation in Go, providing HTTP/REST APIs for vehicle component diagnostics and data retrieval.

## Overview

This SOVD server implements the SOVD specification with a focus on the VIN read use case. It provides mock data for multiple vehicle components and is structured to easily integrate with a UDS (Unified Diagnostic Services) library for real vehicle communication.

## Features

- **RESTful API** following SOVD specification
- **OpenAPI 3.0 specification** for API documentation
- **Mock data** for multiple vehicle components (Engine, Transmission, ABS, Airbag, BCM)
- **Component discovery** - list all available vehicle components
- **Data item discovery** - list available data items per component with category filtering
- **Data retrieval** - get specific data item values (e.g., VIN, software version)
- **Structured for UDS integration** - ready to integrate with C++ UDS library
- **CORS support** for web applications

## Project Structure

```
sovd-server/
├── main.go                     # Application entry point
├── go.mod                      # Go module definition
├── openapi.yaml               # OpenAPI 3.0 specification
├── internal/
│   ├── handlers/
│   │   └── sovd_handler.go    # HTTP request handlers
│   ├── models/
│   │   └── models.go          # Data models and structs
│   └── services/
│       └── sovd_service.go    # Business logic and mock data
└── pkg/
    └── uds/
        └── uds_client.go      # UDS client placeholder for future integration
```

## Quick Start

### Prerequisites

- Go 1.21 or later
- Git (for dependency management)

### Installation and Running

1. **Clone/Navigate to the project directory:**
   ```powershell
   cd "c:\Users\mnaffeti.ACTIA\Desktop\SOVD"
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

The project is structured to easily integrate with your C++ UDS library:

### Integration Points

1. **UDS Client Structure** - `pkg/uds/uds_client.go` contains placeholder structures
2. **Service Layer** - `internal/services/sovd_service.go` contains mock data that can be replaced with UDS calls
3. **CGO Integration** - Ready for C++ library binding

### Integration Steps

1. **Create CGO bindings** for your C++ UDS library
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
Use the provided curl examples or any HTTP client (Postman, Insomnia, etc.)

### Automated Testing
```powershell
# Run tests (when implemented)
go test ./...
```

## Configuration

The server currently runs on port 8080. To change the port, modify the `router.Run(":8080")` line in `main.go`.

## Troubleshooting

### Common Issues

1. **Port already in use**
   - Change the port in `main.go` or stop the conflicting service

2. **Module dependencies**
   - Run `go mod tidy` to resolve dependencies

3. **CORS issues**
   - CORS is already configured for `*` origin in development

## Contributing

1. Follow Go coding standards
2. Add tests for new features
3. Update documentation for API changes
4. Use meaningful commit messages

## License

MIT License - see LICENSE file for details.

## Support

For questions or issues:
- Create GitHub issues for bugs/feature requests
- Check the OpenAPI specification at `/openapi.yaml`
- Use the health check endpoint to verify server status