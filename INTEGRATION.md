# SOVD2UDS Integration Guide

This guide explains how the Rust SOVD2UDS adapter is integrated with the Go SOVD server to enable real UDS protocol communication.

## Architecture Overview

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

## Component Responsibilities

### Go SOVD Server (Port 8080)
- **Primary API**: Exposes SOVD REST API to clients
- **Adapter Integration**: Routes UDS requests to Rust adapter via HTTP
- **Fallback Mechanism**: Uses mock data if adapter unavailable
- **Service Orchestration**: Manages business logic and request validation

**Key Files:**
- `main.go` - Initializes service with adapter URL from environment
- `internal/services/sovd_service.go` - Core business logic with adapter integration
- `pkg/uds/adapter_client.go` - HTTP client for Rust adapter communication

### Rust SOVD2UDS Adapter (Port 8081)
- **Protocol Translation**: Converts SOVD REST calls to UDS protocol
- **UDS Communication**: Interacts with vehicle ECUs via UDS/DoIP
- **FFI Layer**: Manages C library bindings for UDS communication
- **Error Handling**: Translates UDS errors to HTTP responses

**Key Components:**
- `src/server/mod.rs` - Axum HTTP server
- `src/translation/mod.rs` - SOVD to UDS mapping
- `src/uds/client.rs` - UDS client abstraction
- `src/ffi/` - C library bindings

## Integration Points

### 1. Service Initialization

**Environment Variable:**
```bash
SOVD_ADAPTER_URL=http://localhost:8081
```

**Go Code (main.go):**
```go
adapterURL := os.Getenv("SOVD_ADAPTER_URL")
if adapterURL == "" {
    adapterURL = "http://localhost:8081"
}
sovdService := services.NewSOVDService(adapterURL)
```

The Go server:
1. Reads adapter URL from environment (defaults to localhost:8081)
2. Passes URL to service constructor
3. Service performs health check on initialization
4. Logs connection status (connected/fallback mode)

### 2. Data Item Reading

**Client Request:**
```
GET /api/v1/components/ecu_engine/data/engine_speed
```

**Go Service Flow:**
```go
func (s *SOVDService) GetDataItemValue(componentID, dataID string) (*models.DataItemValue, error) {
    // Try adapter first
    if s.useAdapter {
        value, err := s.adapterClient.ReadDataItem(componentID, dataID)
        if err == nil {
            return s.convertAdapterValue(value), nil
        }
        log.Printf("Adapter failed, using mock: %v", err)
    }
    
    // Fall back to mock data
    return s.getMockDataItemValue(componentID, dataID)
}
```

**Adapter Request:**
```
POST http://localhost:8081/api/sovd/read
Content-Type: application/json

{
    "component_id": "ecu_engine",
    "data_identifier": "engine_speed"
}
```

**Adapter Processing:**
1. Maps SOVD data ID to UDS DID (e.g., "engine_speed" → 0xF40D)
2. Sends UDS 0x22 ReadDataByIdentifier request
3. Parses UDS response
4. Returns SOVD-formatted JSON

### 3. DTC Management

**Client Request:**
```
POST /api/v1/components/ecu_engine/dtc/clear
```

**Go Service Flow:**
```go
func (s *SOVDService) ManageDTCs(componentID, operation string) (*models.DTCResponse, error) {
    if s.useAdapter {
        return s.adapterClient.ManageDTCs(componentID, operation)
    }
    return s.getMockDTCs(componentID, operation)
}
```

**Adapter Request:**
```
POST http://localhost:8081/api/sovd/dtc
Content-Type: application/json

{
    "component_id": "ecu_engine",
    "operation": "clear"
}
```

### 4. Actuator Control

**Client Request:**
```
POST /api/v1/components/ecu_body/actuators/door_lock
{
    "action": "lock",
    "parameters": {
        "doors": ["front_left", "front_right"]
    }
}
```

**Adapter Request:**
```
POST http://localhost:8081/api/sovd/control
Content-Type: application/json

{
    "component_id": "ecu_body",
    "actuator_id": "door_lock",
    "action": "lock",
    "parameters": {
        "doors": ["front_left", "front_right"]
    }
}
```

## Health Monitoring

### Adapter Health Check

**Endpoint:**
```
GET http://localhost:8081/health
```

**Response (Healthy):**
```json
{
    "status": "healthy",
    "timestamp": "2024-01-15T10:30:00Z",
    "uds_available": true,
    "capabilities": ["read", "write", "dtc", "control", "session"]
}
```

**Response (Degraded):**
```json
{
    "status": "degraded",
    "timestamp": "2024-01-15T10:30:00Z",
    "uds_available": false,
    "error": "UDS library not initialized"
}
```

### Integration Monitoring

The Go service logs adapter status:
```
INFO: SOVD2UDS adapter connected at http://localhost:8081
INFO: Using real UDS communication via adapter
```

Or when in fallback mode:
```
WARN: SOVD2UDS adapter unavailable, using mock data
WARN: Adapter failed for engine_speed, using mock data
```

## Error Handling

### Adapter Errors

When the adapter returns an error, the Go service falls back gracefully:

```go
value, err := s.adapterClient.ReadDataItem(componentID, dataID)
if err != nil {
    log.Printf("Adapter error for %s/%s: %v", componentID, dataID, err)
    // Fall back to mock data
    return s.getMockDataItemValue(componentID, dataID)
}
```

### UDS Protocol Errors

The adapter translates UDS negative responses to HTTP errors:

**UDS Negative Response:**
```
NRC 0x31 (requestOutOfRange)
```

**HTTP Response:**
```json
{
    "error": "request_out_of_range",
    "message": "Data identifier not supported",
    "uds_code": "0x31"
}
```

## Configuration

### Go Server Configuration

**Environment Variables:**
- `SOVD_ADAPTER_URL` - Adapter endpoint (default: http://localhost:8081)
- `PORT` - Server port (default: 8080)
- `GIN_MODE` - Gin mode (release/debug)

### Rust Adapter Configuration

**config.toml:**
```toml
[server]
host = "127.0.0.1"
port = 8081

[uds]
library_path = "path/to/libudsclient.so"
timeout_ms = 1000
max_retries = 3

[logging]
level = "info"
format = "json"
```

**Environment Variables:**
- `RUST_LOG` - Log level (trace, debug, info, warn, error)
- `CONFIG_PATH` - Path to config file

## Development Workflow

### 1. Start Both Services

**Using PowerShell Script (Recommended):**
```powershell
.\start-integrated.ps1
```

This script:
- Builds both services if needed
- Starts Rust adapter on port 8081
- Waits for adapter to be healthy
- Starts Go server on port 8080
- Handles cleanup on Ctrl+C

**Manual Startup:**

Terminal 1 (Rust Adapter):
```bash
cd sovd2uds-adapter
cargo run --release
```

Terminal 2 (Go Server):
```bash
set SOVD_ADAPTER_URL=http://localhost:8081
go run main.go
```

### 2. Test Integration

**Health Check:**
```bash
curl http://localhost:8080/health
```

**Test Data Reading:**
```bash
curl http://localhost:8080/api/v1/components/ecu_engine/data/engine_speed
```

**Verify Adapter Call:**
Check logs for:
```
INFO: Using adapter for ecu_engine/engine_speed
```

### 3. Develop with Mock Fallback

To develop without the adapter running:
1. Don't start the adapter
2. Go server will automatically use mock data
3. Logs will show: "Adapter unavailable, using mock data"

This allows frontend development without UDS hardware.

## Testing

### Unit Tests

**Go Service Tests:**
```bash
go test ./internal/services/... -v
```

**Rust Adapter Tests:**
```bash
cd sovd2uds-adapter
cargo test
```

### Integration Tests

**Test Script (test-integrated.ps1):**
```powershell
# Test with adapter
Start-Process .\start-integrated.ps1
Start-Sleep 5
Invoke-WebRequest http://localhost:8080/health
Invoke-WebRequest http://localhost:8080/api/v1/components
```

### Manual Testing

**Using the test-api.bat script:**
```bash
.\test-api.bat
```

This tests all SOVD endpoints including those using the adapter.

## Deployment

### Docker Deployment

Both services can be containerized:

**docker-compose.yml:**
```yaml
version: '3.8'
services:
  sovd-adapter:
    build: ./sovd2uds-adapter
    ports:
      - "8081:8081"
    volumes:
      - /path/to/uds/libs:/usr/local/lib
    environment:
      - RUST_LOG=info
    
  sovd-server:
    build: .
    ports:
      - "8080:8080"
    environment:
      - SOVD_ADAPTER_URL=http://sovd-adapter:8081
    depends_on:
      - sovd-adapter
```

### Production Considerations

1. **Load Balancing**: Run multiple adapter instances behind a load balancer
2. **Health Monitoring**: Monitor both services' /health endpoints
3. **Logging**: Centralize logs from both services
4. **Metrics**: Expose Prometheus metrics for monitoring
5. **Security**: Add authentication between services in production

## Troubleshooting

### Adapter Connection Failed

**Symptom:**
```
WARN: SOVD2UDS adapter unavailable, using mock data
```

**Solutions:**
1. Check if adapter is running: `curl http://localhost:8081/health`
2. Verify SOVD_ADAPTER_URL environment variable
3. Check firewall settings
4. Review adapter logs for startup errors

### UDS Communication Errors

**Symptom:**
```json
{
    "error": "uds_error",
    "message": "Failed to communicate with ECU"
}
```

**Solutions:**
1. Verify UDS library path in adapter config
2. Check CAN/DoIP connection to vehicle
3. Verify ECU is powered and accessible
4. Check UDS timeout settings

### Data Format Mismatches

**Symptom:**
```
ERROR: Failed to convert adapter response
```

**Solutions:**
1. Verify data ID mapping in translation layer
2. Check response format from adapter
3. Review stringToTimePtr helper for timestamp conversion
4. Add detailed logging to conversion functions

## Future Enhancements

### Planned Features

1. **Caching**: Cache frequently requested data items
2. **Batching**: Batch multiple UDS requests for efficiency
3. **Async Operations**: Support long-running diagnostic operations
4. **Event Streaming**: WebSocket support for real-time data
5. **Service Discovery**: Auto-discover adapter instances

### Performance Optimization

1. **Connection Pooling**: Reuse HTTP connections to adapter
2. **Request Coalescing**: Combine multiple requests
3. **Circuit Breaker**: Fast-fail when adapter is down
4. **Metrics**: Track adapter response times and error rates

## Related Documentation

- [README.md](README.md) - Project overview
- [sovd2uds-adapter/README.md](sovd2uds-adapter/README.md) - Adapter details
- [sovd2uds-adapter/ARCHITECTURE.md](sovd2uds-adapter/ARCHITECTURE.md) - Adapter architecture
- [SERVICE_EXTENSION_GUIDE.md](SERVICE_EXTENSION_GUIDE.md) - Extending services
- [openapi.yaml](openapi.yaml) - API specification
