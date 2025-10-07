# Integration Guide: Connecting Go SOVD Server with Rust SOVD2UDS Adapter

This guide shows how to integrate the Rust SOVD2UDS adapter with your existing Go SOVD server.

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    Client Application                        │
└───────────────────────────┬─────────────────────────────────┘
                            │ HTTP/REST
                            ▼
┌─────────────────────────────────────────────────────────────┐
│              Go SOVD Server (Port 8080)                     │
│  • Serves OpenAPI endpoints                                 │
│  • Business logic layer                                     │
│  • Forwards UDS operations to Rust adapter                  │
└───────────────────────────┬─────────────────────────────────┘
                            │ HTTP (internal)
                            ▼
┌─────────────────────────────────────────────────────────────┐
│         Rust SOVD2UDS Adapter (Port 8081)                   │
│  • Protocol translation (SOVD ↔ UDS)                        │
│  • FFI to C libraries                                       │
│  • Vehicle communication                                    │
└───────────────────────────┬─────────────────────────────────┘
                            │ UDS/DoIP
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                    Vehicle ECUs                             │
└─────────────────────────────────────────────────────────────┘
```

## Step 1: Update Go SOVD Server

### 1.1 Create UDS Client Wrapper

Create `pkg/uds/adapter_client.go`:

```go
package uds

import (
    "bytes"
    "encoding/json"
    "fmt"
    "io"
    "net/http"
    "time"
)

// AdapterClient wraps the Rust SOVD2UDS adapter
type AdapterClient struct {
    baseURL string
    client  *http.Client
}

// NewAdapterClient creates a new adapter client
func NewAdapterClient(baseURL string) *AdapterClient {
    return &AdapterClient{
        baseURL: baseURL,
        client: &http.Client{
            Timeout: 30 * time.Second,
        },
    }
}

// DataItemValue represents a data item response
type DataItemValue struct {
    ID        string      `json:"id"`
    Name      string      `json:"name"`
    Category  string      `json:"category"`
    Data      interface{} `json:"data"`
    Timestamp string      `json:"timestamp,omitempty"`
    Quality   string      `json:"quality,omitempty"`
}

// ReadDataItem reads a data item from a component
func (c *AdapterClient) ReadDataItem(componentID, dataID string) (*DataItemValue, error) {
    url := fmt.Sprintf("%s/api/v1/components/%s/data/%s", c.baseURL, componentID, dataID)
    
    resp, err := c.client.Get(url)
    if err != nil {
        return nil, fmt.Errorf("failed to call adapter: %w", err)
    }
    defer resp.Body.Close()

    if resp.StatusCode != http.StatusOK {
        body, _ := io.ReadAll(resp.Body)
        return nil, fmt.Errorf("adapter error: %s", string(body))
    }

    var result DataItemValue
    if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
        return nil, fmt.Errorf("failed to decode response: %w", err)
    }

    return &result, nil
}

// DTCManagementRequest represents a DTC operation request
type DTCManagementRequest struct {
    Action string   `json:"action"`
    DTCs   []string `json:"dtcs,omitempty"`
}

// DTCManagementResponse represents a DTC operation response
type DTCManagementResponse struct {
    Action    string      `json:"action"`
    Status    string      `json:"status"`
    Results   interface{} `json:"results,omitempty"`
    Message   string      `json:"message,omitempty"`
    Timestamp string      `json:"timestamp,omitempty"`
}

// ManageDTCs performs DTC operations (read, clear, freeze_frame)
func (c *AdapterClient) ManageDTCs(componentID string, request *DTCManagementRequest) (*DTCManagementResponse, error) {
    url := fmt.Sprintf("%s/api/v1/components/%s/dtcs", c.baseURL, componentID)
    
    payload, err := json.Marshal(request)
    if err != nil {
        return nil, fmt.Errorf("failed to marshal request: %w", err)
    }

    resp, err := c.client.Post(url, "application/json", bytes.NewBuffer(payload))
    if err != nil {
        return nil, fmt.Errorf("failed to call adapter: %w", err)
    }
    defer resp.Body.Close()

    if resp.StatusCode != http.StatusOK {
        body, _ := io.ReadAll(resp.Body)
        return nil, fmt.Errorf("adapter error: %s", string(body))
    }

    var result DTCManagementResponse
    if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
        return nil, fmt.Errorf("failed to decode response: %w", err)
    }

    return &result, nil
}

// Health checks if the adapter is running
func (c *AdapterClient) Health() error {
    resp, err := c.client.Get(c.baseURL + "/health")
    if err != nil {
        return err
    }
    defer resp.Body.Close()

    if resp.StatusCode != http.StatusOK {
        return fmt.Errorf("adapter unhealthy: status %d", resp.StatusCode)
    }

    return nil
}
```

### 1.2 Update SOVD Service

Update `internal/services/sovd_service.go`:

```go
package services

import (
    "fmt"
    "sovd-server/internal/models"
    "sovd-server/pkg/uds"
)

type SOVDService struct {
    adapterClient *uds.AdapterClient
}

func NewSOVDService(adapterURL string) *SOVDService {
    return &SOVDService{
        adapterClient: uds.NewAdapterClient(adapterURL),
    }
}

// GetDataItemValue retrieves a data item value via the Rust adapter
func (s *SOVDService) GetDataItemValue(componentID, dataID string) (*models.DataItemValue, error) {
    // Call Rust adapter
    result, err := s.adapterClient.ReadDataItem(componentID, dataID)
    if err != nil {
        return nil, fmt.Errorf("failed to read data item: %w", err)
    }

    // Convert to internal model
    return &models.DataItemValue{
        ID:        result.ID,
        Name:      result.Name,
        Category:  result.Category,
        Data:      result.Data,
        Timestamp: result.Timestamp,
        Quality:   result.Quality,
    }, nil
}

// ReadDTCs reads DTCs from a component
func (s *SOVDService) ReadDTCs(componentID string) (interface{}, error) {
    request := &uds.DTCManagementRequest{
        Action: "read",
    }

    result, err := s.adapterClient.ManageDTCs(componentID, request)
    if err != nil {
        return nil, fmt.Errorf("failed to read DTCs: %w", err)
    }

    return result.Results, nil
}

// ClearDTCs clears DTCs from a component
func (s *SOVDService) ClearDTCs(componentID string) error {
    request := &uds.DTCManagementRequest{
        Action: "clear",
    }

    _, err := s.adapterClient.ManageDTCs(componentID, request)
    if err != nil {
        return fmt.Errorf("failed to clear DTCs: %w", err)
    }

    return nil
}

// CheckAdapterHealth verifies the adapter is running
func (s *SOVDService) CheckAdapterHealth() error {
    return s.adapterClient.Health()
}
```

### 1.3 Update Main Application

Update `main.go`:

```go
package main

import (
    "log"
    "os"
    "sovd-server/internal/handlers"
    "sovd-server/internal/services"

    "github.com/gin-gonic/gin"
)

func main() {
    // Get adapter URL from environment or use default
    adapterURL := os.Getenv("SOVD_ADAPTER_URL")
    if adapterURL == "" {
        adapterURL = "http://localhost:8081"
    }

    // Initialize service with adapter client
    service := services.NewSOVDService(adapterURL)

    // Check adapter health on startup
    if err := service.CheckAdapterHealth(); err != nil {
        log.Printf("Warning: SOVD2UDS adapter not available: %v", err)
        log.Printf("Make sure the adapter is running on %s", adapterURL)
    } else {
        log.Printf("Successfully connected to SOVD2UDS adapter at %s", adapterURL)
    }

    // Initialize handler
    handler := handlers.NewSOVDHandler(service)

    // Setup router
    router := gin.Default()

    // API routes
    api := router.Group("/api/v1")
    {
        api.GET("/components", handler.GetComponents)
        api.GET("/components/:component_id/data", handler.GetComponentData)
        api.GET("/components/:component_id/data/:data_id", handler.GetDataItemValue)
        api.POST("/components/:component_id/dtcs", handler.ManageDTCs)
    }

    // Health check
    router.GET("/health", func(c *gin.Context) {
        c.JSON(200, gin.H{
            "status":  "healthy",
            "adapter": adapterURL,
        })
    })

    log.Println("Starting SOVD Server on :8080")
    if err := router.Run(":8080"); err != nil {
        log.Fatal(err)
    }
}
```

## Step 2: Configuration

### 2.1 Environment Variables

Create `.env` file:

```bash
# Go SOVD Server
PORT=8080

# Rust SOVD2UDS Adapter URL
SOVD_ADAPTER_URL=http://localhost:8081
```

### 2.2 Docker Compose (Optional)

Create `docker-compose.yml`:

```yaml
version: '3.8'

services:
  sovd-server:
    build: ./sovd-server
    ports:
      - "8080:8080"
    environment:
      - SOVD_ADAPTER_URL=http://sovd-adapter:8081
    depends_on:
      - sovd-adapter

  sovd-adapter:
    build: ./sovd2uds-adapter
    ports:
      - "8081:8081"
    volumes:
      - ./config.toml:/app/config.toml
    environment:
      - RUST_LOG=info
```

## Step 3: Testing the Integration

### 3.1 Start the Adapter

```bash
cd sovd2uds-adapter
./target/release/sovd2uds-adapter
```

### 3.2 Start the Go Server

```bash
cd sovd-server
export SOVD_ADAPTER_URL=http://localhost:8081
go run main.go
```

### 3.3 Test the Flow

```bash
# Test via Go server (port 8080)
curl http://localhost:8080/api/v1/components/engine/data/vin

# This will:
# 1. Hit Go server on 8080
# 2. Go server forwards to Rust adapter on 8081
# 3. Rust adapter calls UDS via FFI
# 4. Response flows back through the chain
```

## Step 4: Error Handling

### 4.1 Handle Adapter Unavailability

```go
func (s *SOVDService) GetDataItemValue(componentID, dataID string) (*models.DataItemValue, error) {
    result, err := s.adapterClient.ReadDataItem(componentID, dataID)
    if err != nil {
        // Check if it's a connection error
        if isConnectionError(err) {
            return nil, fmt.Errorf("SOVD2UDS adapter unavailable: %w", err)
        }
        return nil, err
    }
    return convertToModel(result), nil
}

func isConnectionError(err error) bool {
    // Check for network errors
    return strings.Contains(err.Error(), "connection refused") ||
           strings.Contains(err.Error(), "no such host")
}
```

### 4.2 Implement Retry Logic

```go
func (c *AdapterClient) ReadDataItemWithRetry(componentID, dataID string, maxRetries int) (*DataItemValue, error) {
    var lastErr error
    
    for i := 0; i < maxRetries; i++ {
        result, err := c.ReadDataItem(componentID, dataID)
        if err == nil {
            return result, nil
        }
        
        lastErr = err
        time.Sleep(time.Second * time.Duration(i+1)) // Exponential backoff
    }
    
    return nil, fmt.Errorf("failed after %d retries: %w", maxRetries, lastErr)
}
```

## Step 5: Monitoring

### 5.1 Add Health Check Endpoint

```go
// In Go server
router.GET("/health", func(c *gin.Context) {
    adapterHealthy := true
    if err := service.CheckAdapterHealth(); err != nil {
        adapterHealthy = false
    }

    status := "healthy"
    if !adapterHealthy {
        status = "degraded"
    }

    c.JSON(200, gin.H{
        "status":         status,
        "adapter_status": adapterHealthy,
    })
})
```

### 5.2 Add Logging

```go
import "github.com/sirupsen/logrus"

func (c *AdapterClient) ReadDataItem(componentID, dataID string) (*DataItemValue, error) {
    logrus.WithFields(logrus.Fields{
        "component": componentID,
        "data_item": dataID,
    }).Info("Reading data item via adapter")

    // ... rest of implementation
}
```

## Step 6: Production Deployment

### 6.1 Systemd Services

**Go Server** (`/etc/systemd/system/sovd-server.service`):

```ini
[Unit]
Description=SOVD Server
After=network.target sovd-adapter.service

[Service]
Type=simple
User=sovd
WorkingDirectory=/opt/sovd-server
Environment="SOVD_ADAPTER_URL=http://localhost:8081"
ExecStart=/opt/sovd-server/sovd-server
Restart=always

[Install]
WantedBy=multi-user.target
```

**Rust Adapter** (`/etc/systemd/system/sovd-adapter.service`):

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

### 6.2 Reverse Proxy (Nginx)

```nginx
upstream sovd_server {
    server localhost:8080;
}

server {
    listen 80;
    server_name sovd.example.com;

    location / {
        proxy_pass http://sovd_server;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

## Example Request Flow

```
Client Request:
  GET http://sovd.example.com/api/v1/components/engine/data/vin

Nginx (Port 80):
  → Forwards to localhost:8080

Go SOVD Server (Port 8080):
  → Receives request
  → Calls http://localhost:8081/api/v1/components/engine/data/vin

Rust SOVD2UDS Adapter (Port 8081):
  → Receives request
  → Translates: "vin" → DID 0xF190
  → Calls FFI: uds_read_data_by_identifier(0xF190)
  → libudsclient sends UDS request
  → ECU responds with VIN
  → Formats as JSON
  → Returns to Go server

Go Server:
  → Receives JSON response
  → Returns to client

Client receives:
{
  "id": "vin",
  "name": "Vehicle Identification Number",
  "data": "WVWZZZ1KZBW123456",
  "timestamp": "2025-10-07T10:30:00Z",
  "quality": "good"
}
```

## Troubleshooting

### Adapter Not Responding

```bash
# Check if adapter is running
curl http://localhost:8081/health

# Check Go server connection
curl http://localhost:8080/health
```

### Connection Refused

- Ensure both services are running
- Check firewall rules
- Verify ports are not in use: `netstat -an | grep 808`

### Timeout Issues

- Increase timeout in Go HTTP client
- Increase UDS timeout in adapter config
- Check network connectivity to ECU

## Summary

This integration allows your Go SOVD server to leverage the Rust adapter for all UDS operations while maintaining a clean separation of concerns:

- **Go Server**: Business logic, API orchestration
- **Rust Adapter**: Protocol translation, hardware interface

The architecture is scalable, maintainable, and allows independent deployment of each component.
