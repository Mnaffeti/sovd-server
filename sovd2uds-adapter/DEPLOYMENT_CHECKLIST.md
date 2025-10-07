# SOVD2UDS Adapter - Deployment Checklist

Use this checklist to ensure proper deployment of the SOVD2UDS adapter.

## Pre-Deployment

### ✅ Environment Setup

- [ ] **Rust toolchain installed** (1.70+)
  ```bash
  rustc --version  # Should be 1.70 or higher
  ```

- [ ] **C compiler available**
  ```bash
  gcc --version    # Linux
  clang --version  # macOS
  cl.exe           # Windows (MSVC)
  ```

- [ ] **C libraries available**
  - [ ] libudsclient installed
  - [ ] libdoipclient installed
  - [ ] Header files accessible
  - [ ] Libraries in library path

- [ ] **Environment variables set**
  ```bash
  echo $UDS_INCLUDE_PATH      # Should point to headers
  echo $DOIP_INCLUDE_PATH     # Should point to headers
  echo $LD_LIBRARY_PATH       # Should include lib directories
  ```

### ✅ Build Process

- [ ] **Clean build successful**
  ```bash
  cargo clean
  cargo build --release
  ```

- [ ] **Tests pass**
  ```bash
  cargo test
  ```

- [ ] **Binary created**
  ```bash
  ls -lh target/release/sovd2uds-adapter
  ```

- [ ] **Dependencies resolved**
  ```bash
  ldd target/release/sovd2uds-adapter  # Linux
  otool -L target/release/sovd2uds-adapter  # macOS
  ```

## Configuration

### ✅ config.toml Setup

- [ ] **Server configuration**
  - [ ] Host address set (e.g., "0.0.0.0" for production)
  - [ ] Port configured (default: 8081)
  - [ ] Request timeout appropriate

- [ ] **UDS configuration**
  - [ ] Interface specified (can0, doip, etc.)
  - [ ] Default ECU address set
  - [ ] Timeout configured
  - [ ] Max retries set

- [ ] **DoIP configuration** (if using DoIP)
  - [ ] Enabled flag set
  - [ ] Target IP address configured
  - [ ] Port set (usually 13400)
  - [ ] Source/target addresses configured

- [ ] **Component mappings**
  - [ ] All components mapped to ECU addresses
  - [ ] Addresses verified against vehicle specs

- [ ] **Logging configuration**
  - [ ] Log level appropriate (info for production)
  - [ ] Log format set (json for production)
  - [ ] Log file path specified

- [ ] **Security configuration**
  - [ ] Security access settings reviewed
  - [ ] Security level appropriate

### ✅ Environment Variables (Production)

- [ ] **.env file created** (copy from .env.example)
  ```bash
  cp .env.example .env
  nano .env
  ```

- [ ] **Variables set correctly**
  - [ ] SOVD2UDS__SERVER__HOST
  - [ ] SOVD2UDS__SERVER__PORT
  - [ ] SOVD2UDS__LOGGING__LEVEL
  - [ ] SOVD2UDS__LOGGING__FORMAT

## Testing

### ✅ Functional Testing

- [ ] **Health check works**
  ```bash
  curl http://localhost:8081/health
  ```

- [ ] **Get components works**
  ```bash
  curl http://localhost:8081/api/v1/components
  ```

- [ ] **Read data item works** (with actual ECU)
  ```bash
  curl http://localhost:8081/api/v1/components/engine/data/vin
  ```

- [ ] **DTC operations work**
  ```bash
  curl -X POST http://localhost:8081/api/v1/components/engine/dtcs \
    -H "Content-Type: application/json" \
    -d '{"action": "read"}'
  ```

- [ ] **Error handling works**
  - [ ] Invalid component returns 404
  - [ ] Invalid data item returns 404
  - [ ] Malformed JSON returns 400

### ✅ Integration Testing

- [ ] **Go SOVD server integration tested**
  - [ ] Go server can reach adapter
  - [ ] End-to-end requests work
  - [ ] Error propagation correct

- [ ] **ECU connectivity verified**
  - [ ] CAN interface working (if using CAN)
  - [ ] DoIP connection successful (if using DoIP)
  - [ ] ECU responses received

### ✅ Load Testing (Optional)

- [ ] **Concurrent requests handled**
  ```bash
  # Use Apache Bench or similar
  ab -n 100 -c 10 http://localhost:8081/api/v1/components
  ```

- [ ] **Memory usage acceptable**
  ```bash
  top -p $(pgrep sovd2uds-adapter)
  ```

- [ ] **No memory leaks detected**

## Security

### ✅ Security Review

- [ ] **Network access restricted**
  - [ ] Firewall rules configured
  - [ ] Only necessary ports open
  - [ ] Internal network only (if applicable)

- [ ] **Authentication considered**
  - [ ] Middleware added if needed
  - [ ] API keys configured (if applicable)

- [ ] **Logging security**
  - [ ] No sensitive data in logs
  - [ ] Log files have proper permissions

- [ ] **File permissions correct**
  ```bash
  chmod 755 target/release/sovd2uds-adapter
  chmod 644 config.toml
  ```

## Deployment

### ✅ System Service (Linux)

- [ ] **Systemd service file created**
  ```bash
  sudo nano /etc/systemd/system/sovd2uds-adapter.service
  ```

- [ ] **Service file correct**
  - [ ] User/group set
  - [ ] Working directory set
  - [ ] ExecStart path correct
  - [ ] Environment variables set
  - [ ] Restart policy configured

- [ ] **Service enabled and started**
  ```bash
  sudo systemctl enable sovd2uds-adapter
  sudo systemctl start sovd2uds-adapter
  sudo systemctl status sovd2uds-adapter
  ```

- [ ] **Service logs checked**
  ```bash
  sudo journalctl -u sovd2uds-adapter -f
  ```

### ✅ Docker Deployment (Optional)

- [ ] **Dockerfile created**
- [ ] **Image built successfully**
  ```bash
  docker build -t sovd2uds-adapter .
  ```

- [ ] **Container runs**
  ```bash
  docker run -p 8081:8081 sovd2uds-adapter
  ```

- [ ] **Volumes mounted correctly**
- [ ] **Network configuration correct**

### ✅ Production Checklist

- [ ] **Binary deployed to production server**
- [ ] **Config file deployed**
- [ ] **C libraries available on production**
- [ ] **Service started automatically**
- [ ] **Logs being collected**
- [ ] **Monitoring configured**

## Monitoring & Maintenance

### ✅ Monitoring Setup

- [ ] **Health check endpoint monitored**
  - [ ] Uptime monitoring configured
  - [ ] Alert on downtime set up

- [ ] **Log aggregation configured**
  - [ ] Logs sent to centralized system
  - [ ] Log rotation configured

- [ ] **Metrics collected** (if applicable)
  - [ ] Request rates
  - [ ] Error rates
  - [ ] Latencies

- [ ] **Alerts configured**
  - [ ] High error rate alerts
  - [ ] Service down alerts
  - [ ] Resource usage alerts

### ✅ Backup & Recovery

- [ ] **Configuration backed up**
  ```bash
  cp config.toml config.toml.backup
  ```

- [ ] **Recovery procedure documented**
- [ ] **Rollback plan in place**

### ✅ Documentation

- [ ] **Deployment documentation created**
- [ ] **Runbook created** for operations team
- [ ] **Contact information documented**
- [ ] **Known issues documented**

## Post-Deployment

### ✅ Verification (First 24 Hours)

- [ ] **Service running smoothly**
  ```bash
  sudo systemctl status sovd2uds-adapter
  ```

- [ ] **No error spikes in logs**
  ```bash
  sudo journalctl -u sovd2uds-adapter --since "1 hour ago" | grep ERROR
  ```

- [ ] **Requests being served**
  - [ ] Check access logs
  - [ ] Verify response times

- [ ] **Resource usage normal**
  - [ ] CPU usage acceptable
  - [ ] Memory usage stable
  - [ ] No memory leaks

- [ ] **Integration working**
  - [ ] Go SOVD server communicating successfully
  - [ ] End-to-end workflows functional

### ✅ Handover

- [ ] **Operations team trained**
- [ ] **Documentation handed over**
- [ ] **Support contacts shared**
- [ ] **Escalation procedure defined**

## Troubleshooting Guide

### Common Issues Checklist

**Service won't start:**
- [ ] Check C library dependencies
  ```bash
  ldd target/release/sovd2uds-adapter
  ```
- [ ] Check file permissions
- [ ] Check config file syntax
- [ ] Check port availability
  ```bash
  netstat -tuln | grep 8081
  ```

**Connection errors:**
- [ ] Verify ECU connectivity
- [ ] Check network configuration
- [ ] Verify ECU addresses in config
- [ ] Check firewall rules

**Timeout errors:**
- [ ] Increase UDS timeout in config
- [ ] Check ECU responsiveness
- [ ] Verify network latency

**High memory usage:**
- [ ] Check for connection leaks
- [ ] Review connection pool size
- [ ] Monitor over time for leaks

## Sign-off

### ✅ Final Checklist

- [ ] All tests passed
- [ ] Configuration reviewed
- [ ] Security reviewed
- [ ] Deployment successful
- [ ] Monitoring active
- [ ] Documentation complete
- [ ] Team trained

**Deployed by**: _______________  
**Date**: _______________  
**Environment**: ☐ Development  ☐ Staging  ☐ Production  
**Version**: _______________  

**Approved by**: _______________  
**Date**: _______________  

---

## Notes

Use this space to document any deployment-specific notes, customizations, or issues encountered:

```
_________________________________________________________________

_________________________________________________________________

_________________________________________________________________

_________________________________________________________________
```

---

**Checklist Version**: 1.0  
**Last Updated**: 2025-10-07
