# Quick Start Guide - SOVD2UDS Adapter

This guide will help you get the SOVD2UDS adapter up and running quickly.

## Prerequisites

✅ **Rust** 1.70+ installed ([rustup.rs](https://rustup.rs))  
✅ **C Compiler** (GCC, Clang, or MSVC)  
✅ **libudsclient** and **libdoipclient** C libraries  

## Step 1: Configure C Library Paths

### Linux/macOS
```bash
export UDS_INCLUDE_PATH=/path/to/libudsclient/include
export DOIP_INCLUDE_PATH=/path/to/libdoipclient/include
export LD_LIBRARY_PATH=/path/to/libs:$LD_LIBRARY_PATH
```

### Windows (PowerShell)
```powershell
$env:UDS_INCLUDE_PATH = "C:\path\to\libudsclient\include"
$env:DOIP_INCLUDE_PATH = "C:\path\to\libdoipclient\include"
$env:PATH = "C:\path\to\libs;$env:PATH"
```

## Step 2: Build the Adapter

### Using Build Script (Recommended)

**Linux/macOS:**
```bash
chmod +x build.sh
./build.sh release
```

**Windows:**
```powershell
.\build.ps1 -BuildType release
```

### Manual Build

```bash
cargo build --release
```

## Step 3: Configure the Adapter

Edit `config.toml`:

```toml
[server]
host = "127.0.0.1"
port = 8081

[uds]
interface = "can0"          # or "doip" for DoIP
default_address = 0x7E0

[doip]
enabled = true
target_address = "192.168.1.100"  # Your vehicle/ECU IP

[components]
engine = 0x7E0              # Map your ECU addresses
transmission = 0x7E1
```

## Step 4: Run the Adapter

```bash
./target/release/sovd2uds-adapter
```

You should see:
```
Starting SOVD2UDS Adapter v0.1.0
Server listening on http://127.0.0.1:8081
SOVD2UDS Adapter is ready to accept connections
```

## Step 5: Test the API

### Check Health
```bash
curl http://localhost:8081/health
```

### Get Components
```bash
curl http://localhost:8081/api/v1/components
```

### Read VIN
```bash
curl http://localhost:8081/api/v1/components/engine/data/vin
```

### Read DTCs
```bash
curl -X POST http://localhost:8081/api/v1/components/engine/dtcs \
  -H "Content-Type: application/json" \
  -d '{"action": "read"}'
```

## Step 6: Integrate with SOVD Server

Update your Go SOVD server to call the adapter:

```go
// In your Go SOVD server
resp, err := http.Get("http://localhost:8081/api/v1/components/engine/data/vin")
if err != nil {
    return err
}
defer resp.Body.Close()

var result DataItemValue
json.NewDecoder(resp.Body).Decode(&result)
fmt.Printf("VIN: %s\n", result.Data)
```

## Common Issues

### "Failed to create UDS client"
- **Solution**: Check C library paths in environment variables
- Verify libraries are accessible (check `LD_LIBRARY_PATH` or `PATH`)

### "Component not found"
- **Solution**: Add component to `config.toml` under `[components]`

### Compilation errors with bindgen
- **Solution**: Install LLVM/Clang:
  ```bash
  # Ubuntu/Debian
  sudo apt install llvm-dev libclang-dev clang
  
  # macOS
  brew install llvm
  
  # Windows
  # Install Visual Studio with C++ tools
  ```

### Connection timeout
- **Solution**: 
  - Check vehicle/ECU network connectivity
  - Verify ECU address is correct
  - Increase timeout in `config.toml`: `uds.timeout = 10000`

## Next Steps

✅ **Read the full [README.md](README.md)** for detailed documentation  
✅ **Check [ARCHITECTURE.md](ARCHITECTURE.md)** to understand the design  
✅ **Run tests**: `python tests/test_api.py`  
✅ **Customize DID mappings** in `src/translation/mod.rs`  
✅ **Add your components** to `config.toml`  

## Example Workflow

```bash
# 1. Build
./build.sh release

# 2. Configure
nano config.toml  # Edit ECU addresses

# 3. Run
./target/release/sovd2uds-adapter

# 4. Test (in another terminal)
python tests/test_api.py

# 5. Integrate with your SOVD server
# Update your Go code to call http://localhost:8081/api/v1/...
```

## Production Deployment

For production use:

1. **Use release build**: `cargo build --release`
2. **Configure logging**: Set `logging.format = "json"` in config.toml
3. **Set resource limits**: Configure `max_concurrent_requests`
4. **Enable monitoring**: Add health checks to your monitoring system
5. **Secure the service**: Add authentication middleware if needed
6. **Use systemd/Docker**: Run as a service

### Example systemd service

```ini
[Unit]
Description=SOVD2UDS Adapter
After=network.target

[Service]
Type=simple
User=sovd
WorkingDirectory=/opt/sovd2uds-adapter
ExecStart=/opt/sovd2uds-adapter/target/release/sovd2uds-adapter
Restart=always

[Install]
WantedBy=multi-user.target
```

## Support

- 📖 Full docs: [README.md](README.md)
- 🏗️ Architecture: [ARCHITECTURE.md](ARCHITECTURE.md)
- 🐛 Issues: Create a GitHub issue
- 💬 Questions: support@example.com

**Happy diagnostics! 🚗💨**
