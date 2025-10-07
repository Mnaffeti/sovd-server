# SOVD2UDS Adapter Architecture

## Overview

The SOVD2UDS adapter is a sophisticated bridge that translates between modern Service-Oriented Vehicle Diagnostics (SOVD) REST API and legacy Unified Diagnostic Services (UDS) protocol.

## System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        SOVD Server (Go)                         │
│                     REST API Consumer                           │
└────────────────────────────┬────────────────────────────────────┘
                             │
                             │ HTTP/REST (JSON)
                             │
┌────────────────────────────▼────────────────────────────────────┐
│                  SOVD2UDS Adapter (Rust)                        │
│                                                                  │
│  ┌────────────────────────────────────────────────────────┐   │
│  │              REST API Layer (Axum)                     │   │
│  │  • HTTP server on port 8081                            │   │
│  │  • Request routing and validation                      │   │
│  │  • JSON serialization/deserialization                  │   │
│  │  • Error handling and HTTP status codes                │   │
│  └──────────────────────┬─────────────────────────────────┘   │
│                         │                                       │
│  ┌──────────────────────▼─────────────────────────────────┐   │
│  │           Translation Layer                            │   │
│  │  • SOVD data item → UDS DID mapping                   │   │
│  │  • SOVD operations → UDS services mapping             │   │
│  │  • Data format conversion (JSON ↔ bytes)              │   │
│  │  • DTC parsing and formatting                         │   │
│  └──────────────────────┬─────────────────────────────────┘   │
│                         │                                       │
│  ┌──────────────────────▼─────────────────────────────────┐   │
│  │        UDS Client Abstraction Layer                    │   │
│  │  • High-level UDS operations API                      │   │
│  │  • Connection pool management                         │   │
│  │  • Async/await interface                              │   │
│  │  • Security access handling                           │   │
│  │  • Session management                                 │   │
│  └──────────────────────┬─────────────────────────────────┘   │
│                         │                                       │
│  ┌──────────────────────▼─────────────────────────────────┐   │
│  │            FFI Bindings Layer                          │   │
│  │  • Safe Rust wrappers around C functions              │   │
│  │  • Memory management (Rust ↔ C)                       │   │
│  │  • Error code translation                             │   │
│  │  • Thread safety guarantees                           │   │
│  └──────────────────────┬─────────────────────────────────┘   │
│                         │                                       │
└─────────────────────────┼───────────────────────────────────────┘
                          │ C ABI
        ┌─────────────────┴──────────────────┐
        │                                     │
┌───────▼──────────┐              ┌──────────▼──────────┐
│  libudsclient    │              │   libdoipclient     │
│  (C Library)     │◄────────────►│   (C Library)       │
│                  │              │                     │
│  • UDS protocol  │              │  • DoIP protocol    │
│  • Service impl  │              │  • IP/TCP layer     │
│  • Request/resp  │              │  • Message routing  │
└───────┬──────────┘              └─────────────────────┘
        │
        │ CAN/DoIP
        │
┌───────▼──────────────────────────────────────────────┐
│              Vehicle Network                          │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐│
│  │  Engine │  │  Trans  │  │   ABS   │  │ Airbag  ││
│  │   ECU   │  │   ECU   │  │   ECU   │  │   ECU   ││
│  └─────────┘  └─────────┘  └─────────┘  └─────────┘│
└───────────────────────────────────────────────────────┘
```

## Data Flow

### Example: Read VIN Request

```
1. HTTP Request
   GET /api/v1/components/engine/data/vin
   ↓

2. Axum Handler (handlers.rs)
   - Parse component_id: "engine"
   - Parse data_id: "vin"
   - Validate request
   ↓

3. Translation Layer (translation/mod.rs)
   - Lookup: "vin" → DID 0xF190
   - Get UDS client for "engine" component
   ↓

4. UDS Client (uds/client.rs)
   - Get ECU address: engine → 0x7E0
   - Prepare UDS request: ReadDataByIdentifier
   ↓

5. FFI Layer (ffi/bindings.rs)
   - Call: uds_read_data_by_identifier(client, 0xF190)
   - Handle unsafe C interaction
   ↓

6. libudsclient (C)
   - Build UDS message: [22 F1 90]
   - Send via DoIP/CAN
   ↓

7. Vehicle ECU
   - Process UDS request
   - Respond: [62 F1 90 ...VIN bytes...]
   ↓

8. Response Processing (reverse flow)
   libudsclient → FFI → UDS Client → Translation → Handler
   - Parse UDS response
   - Convert to UTF-8 string
   - Format as SOVD JSON
   ↓

9. HTTP Response
   {
     "id": "vin",
     "name": "Vehicle Identification Number",
     "category": "identData",
     "data": "WVWZZZ1KZBW123456",
     "timestamp": "2025-10-07T10:30:00Z",
     "quality": "good"
   }
```

## Component Details

### 1. REST API Layer

**Technology**: Axum (Rust web framework)

**Responsibilities**:
- HTTP server lifecycle management
- Request routing
- Request/response serialization
- CORS handling
- Error responses

**Key Files**:
- `src/server/handlers.rs`: Request handlers
- `src/models/sovd.rs`: SOVD data models

### 2. Translation Layer

**Responsibilities**:
- Protocol mapping (SOVD ↔ UDS)
- Data format conversion
- DID management
- Category-based filtering

**Key Mappings**:

| SOVD Data Item | UDS DID | Category |
|---------------|---------|----------|
| vin | 0xF190 | identData |
| ecu_software_version | 0xF194 | identData |
| ecu_hardware_version | 0xF191 | identData |

**Key Files**:
- `src/translation/mod.rs`: Translation logic

### 3. UDS Client Abstraction

**Responsibilities**:
- High-level UDS API
- Connection management
- Session handling
- Security access
- Async operations

**Key Features**:
- Connection pooling (reuse ECU connections)
- Automatic retry logic
- Timeout management
- Thread-safe (Arc + RwLock)

**Key Files**:
- `src/uds/client.rs`: UDS client implementation

### 4. FFI Bindings Layer

**Technology**: Rust bindgen

**Responsibilities**:
- Safe wrappers around C functions
- Memory management
- Error handling
- Type conversions

**Safety Considerations**:
- All C interactions wrapped in `unsafe` blocks
- Null pointer checks
- Resource cleanup (Drop trait)
- Memory ownership tracking

**Key Files**:
- `src/ffi/bindings.rs`: FFI wrappers
- `build.rs`: Bindgen configuration
- `wrapper.h`: C header declarations

### 5. Configuration Management

**Format**: TOML with environment variable overrides

**Hierarchy**:
1. Default values (in code)
2. `config.toml` file
3. Environment variables (prefix: `SOVD2UDS__`)

**Key Files**:
- `src/config/mod.rs`: Configuration structures
- `config.toml`: Configuration file

### 6. Error Handling

**Strategy**: Result-based with custom error types

**Error Types**:
- `Sovd2UdsError`: Main error enum
- UDS negative response codes (NRC)
- HTTP status mapping

**Error Propagation**:
```
C error code
  ↓ FFI layer
Rust Result<T, Sovd2UdsError>
  ↓ UDS client
Result<T, Sovd2UdsError>
  ↓ Translation layer
Result<T, Sovd2UdsError>
  ↓ Handler
HTTP StatusCode + JSON error
```

**Key Files**:
- `src/error.rs`: Error definitions

## Concurrency Model

### Async Runtime: Tokio

**Architecture**:
- Multi-threaded runtime
- Async I/O for HTTP and UDS operations
- Connection pooling for ECU connections

**Synchronization**:
- `Arc`: Shared ownership (config, client pool)
- `RwLock`: Reader-writer lock for client access
- Async locks (tokio::sync)

**Thread Safety**:
- FFI handles marked as `Send + Sync`
- Careful use of `unsafe` blocks
- No global mutable state

## Protocol Mappings

### UDS Services Supported

| UDS Service | SID | SOVD Operation |
|------------|-----|----------------|
| DiagnosticSessionControl | 0x10 | Service: session_control |
| ECUReset | 0x11 | Service: ecu_reset |
| ReadDataByIdentifier | 0x22 | GET /data/{data_id} |
| ReadDTCInformation | 0x19 | POST /dtcs (action: read) |
| ClearDiagnosticInformation | 0x14 | POST /dtcs (action: clear) |
| SecurityAccess | 0x27 | Internal (automatic) |
| WriteDataByIdentifier | 0x2E | PUT /data/{data_id} |
| RoutineControl | 0x31 | POST /actuators/control |

### DID Mappings

Common DIDs mapped to SOVD data items:

```rust
"vin" → 0xF190
"ecu_serial_number" → 0xF18C
"ecu_hardware_version" → 0xF191
"ecu_software_version" → 0xF194
"manufacturing_date" → 0xF18B
"system_supplier_id" → 0xF18A
```

## Performance Characteristics

### Latency Breakdown (typical read operation)

```
Total: ~50-100ms
├─ REST handling: 1-2ms
├─ Translation: <1ms
├─ UDS client: 1-2ms
├─ FFI overhead: <1ms
├─ libudsclient: 5-10ms
└─ ECU response: 40-80ms (network + processing)
```

### Scalability

- **Concurrent Requests**: 10+ (configurable)
- **Connection Pool**: 5 connections (configurable)
- **Memory**: ~10-20MB (idle), scales with connections
- **CPU**: Minimal (async I/O)

## Security Architecture

### Security Access Flow

```
1. Write operation requested
2. Check config: require_security_access?
3. If yes:
   a. Request seed (sub-function 0x01)
   b. Calculate key from seed
   c. Send key (sub-function 0x02)
   d. Verify access granted
4. Proceed with operation
```

### Network Security

- Internal service (not exposed externally)
- Add authentication middleware for production
- Use TLS for external exposure
- Firewall rules recommended

## Extension Points

### Adding New DID Mappings

```rust
// In translation/mod.rs
translator.add_did_mapping("custom_data".to_string(), 0x1234);
```

### Adding New Components

```toml
# In config.toml
[components]
custom_ecu = 0x7E4
```

### Custom Security Algorithm

```rust
// In uds/client.rs
fn calculate_security_key(&self, seed: &[u8]) -> Vec<u8> {
    // Implement your security algorithm
    custom_security_algorithm(seed)
}
```

## Deployment Considerations

### Production Checklist

- [ ] Configure proper ECU addresses
- [ ] Set appropriate timeouts
- [ ] Enable structured logging (JSON)
- [ ] Configure log rotation
- [ ] Set resource limits
- [ ] Configure firewall rules
- [ ] Implement monitoring/alerting
- [ ] Test failover scenarios
- [ ] Document security algorithm
- [ ] Backup configuration

### Monitoring

**Key Metrics**:
- Request latency (p50, p95, p99)
- Error rates by type
- Active connections
- UDS timeout rate
- ECU availability

**Tools**:
- Prometheus metrics (future)
- Structured logs (tracing)
- Health check endpoint

## Future Enhancements

1. **WebSocket Support**: Real-time data streaming
2. **GraphQL API**: Alternative to REST
3. **Metrics Export**: Prometheus integration
4. **Circuit Breaker**: Fault tolerance
5. **Request Caching**: Reduce ECU load
6. **Multi-ECU Transactions**: Atomic operations
7. **Plugin System**: Custom protocol extensions

---

**Document Version**: 1.0  
**Last Updated**: 2025-10-07
