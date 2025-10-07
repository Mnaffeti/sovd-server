# SOVD2UDS Integration - Quick Reference

## 🎯 What You Have Now

A **dual-mode SOVD server** that can:
- ✅ Use **real UDS communication** via Rust adapter
- ✅ **Automatically fall back** to mock data when adapter unavailable
- ✅ **Easy startup** with one command
- ✅ **Full documentation** and testing

## 🚀 Quick Start Commands

### Start Everything (Recommended)
```powershell
.\start-integrated.ps1
```
This starts both the Rust adapter (port 8081) and Go server (port 8080).

### Start Go Server Only (Mock Data)
```powershell
go run main.go
```
The server will automatically use mock data if adapter not available.

### Test the Integration
```powershell
.\test-integration.ps1
```
Runs automated tests to verify everything works.

## 📊 System Ports

| Service | Port | URL |
|---------|------|-----|
| Go SOVD Server | 8080 | http://localhost:8080 |
| Rust Adapter | 8081 | http://localhost:8081 |
| Health Check (Go) | 8080 | http://localhost:8080/health |
| Health Check (Rust) | 8081 | http://localhost:8081/health |

## 🔧 Configuration

Set adapter URL via environment variable:
```powershell
$env:SOVD_ADAPTER_URL = "http://localhost:8081"
```

## 📁 Key Files

### New Integration Files
```
├── pkg/uds/adapter_client.go       # HTTP client for Rust adapter
├── start-integrated.ps1             # Startup script (PowerShell)
├── start-integrated.bat             # Startup script (Batch)
├── test-integration.ps1             # Integration tests
├── INTEGRATION.md                   # Detailed integration guide
└── INTEGRATION_SUMMARY.md           # Complete summary
```

### Modified Files
```
├── main.go                          # Now accepts adapter URL
├── internal/services/sovd_service.go # Integrated with adapter
└── README.md                        # Updated documentation
```

## 🧪 Testing Endpoints

### Health Checks
```bash
curl http://localhost:8080/health    # Go server
curl http://localhost:8081/health    # Rust adapter
```

### Data Reading
```bash
# VIN (Vehicle Identification Number)
curl http://localhost:8080/api/v1/components/ecu_engine/data/vin

# Engine Speed
curl http://localhost:8080/api/v1/components/ecu_engine/data/engine_speed

# List all components
curl http://localhost:8080/api/v1/components
```

## 📝 API Examples

### Read Data Item
```bash
GET /api/v1/components/{component_id}/data/{data_id}
```

**Example:**
```bash
curl http://localhost:8080/api/v1/components/ecu_engine/data/engine_speed
```

**Response:**
```json
{
  "id": "engine_speed",
  "name": "Engine Speed",
  "value": "2500",
  "unit": "rpm",
  "timestamp": "2024-01-15T10:30:00Z",
  "quality": "good"
}
```

### Manage DTCs
```bash
POST /api/v1/components/{component_id}/dtc/{operation}
```

**Example:**
```bash
curl -X POST http://localhost:8080/api/v1/components/ecu_engine/dtc/read
```

## 🔄 Request Flow

```
Client → Go Server → Check Adapter Available?
                          ├── Yes → Rust Adapter → UDS → ECU
                          └── No  → Mock Data
```

## 📋 Integration Status

| Feature | Status | Notes |
|---------|--------|-------|
| Data Reading | ✅ Integrated | Via `GetDataItemValue()` |
| DTC Management | 🔄 Ready | Client method created |
| Actuator Control | 🔄 Ready | Client method created |
| Service Execution | 🔄 Ready | Client method created |
| Health Monitoring | ✅ Complete | Both servers |
| Fallback Logic | ✅ Complete | Automatic mock data |
| Documentation | ✅ Complete | 5 docs created |
| Testing | ✅ Complete | Test suite ready |

## 🛠️ Next Steps

1. **Test the Integration**
   ```powershell
   .\start-integrated.ps1
   .\test-integration.ps1
   ```

2. **Build Rust Adapter** (if not already built)
   ```powershell
   cd sovd2uds-adapter
   cargo build --release
   cd ..
   ```

3. **Complete Remaining Integrations**
   - Update `ManageDTCs()` to use adapter
   - Update `ControlActuator()` to use adapter
   - Update `ExecuteService()` to use adapter

4. **Test with Real Vehicle**
   - Configure UDS library path
   - Connect to vehicle CAN/DoIP
   - Verify real UDS communication

## 📚 Documentation

| Document | Description |
|----------|-------------|
| `README.md` | Project overview with integration |
| `INTEGRATION.md` | Detailed integration guide (400+ lines) |
| `INTEGRATION_SUMMARY.md` | Complete implementation summary |
| `QUICK_REFERENCE.md` | This file - quick commands |
| `sovd2uds-adapter/README.md` | Rust adapter documentation |

## 🐛 Troubleshooting

### Adapter Not Connecting
```
WARN: SOVD2UDS adapter unavailable, using mock data
```
**Solutions:**
1. Check if adapter is running: `curl http://localhost:8081/health`
2. Verify `SOVD_ADAPTER_URL` environment variable
3. Build adapter: `cd sovd2uds-adapter && cargo build --release`

### Build Errors
```powershell
go mod tidy  # Update dependencies
go build -o sovd-server.exe .  # Rebuild
```

### Port Already in Use
```powershell
# Windows: Find and kill process on port 8080
netstat -ano | findstr :8080
taskkill /PID <process_id> /F
```

## 💡 Tips

- **Development**: Use mock mode (`go run main.go` only)
- **Testing**: Use integrated mode (`.\start-integrated.ps1`)
- **Production**: Use environment variables for configuration
- **Debugging**: Check logs for "Using adapter" vs "Using mock" messages

## 🎓 Learn More

- Read `INTEGRATION.md` for deep dive on architecture
- Check `sovd2uds-adapter/ARCHITECTURE.md` for adapter internals
- Review `openapi.yaml` for complete API specification

## ✅ Quick Health Check

Run these commands to verify everything works:

```powershell
# 1. Build and start (one command)
.\start-integrated.ps1

# 2. In another terminal, test endpoints
curl http://localhost:8080/health
curl http://localhost:8081/health
curl http://localhost:8080/api/v1/components

# 3. Run automated tests
.\test-integration.ps1
```

If all return `200 OK`, you're ready to go! 🎉
