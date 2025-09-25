# SOVD Server - Service Extension Guide

This guide explains the best practices for adding new service requests to your SOVD server.

## Architecture Overview

The SOVD server follows a layered architecture that makes it easy to extend:

```
HTTP Client → Handlers → Services → UDS Client → Vehicle Network
```

## Adding New Service Types

### 1. **Data-Based Services** (GET operations)
For read-only operations like reading sensor values, identification data, etc.

**Implementation Pattern:**
- Add new data categories in `sovd_service.go`
- Update mock data structures
- No new endpoints needed (uses existing `/data` endpoints)

**Example:** Adding "diagnosticData" category
```go
// In GetComponentDataItems()
{
    ID:          "dtc_count",
    Name:        "Diagnostic Trouble Code Count", 
    Category:    "diagnosticData",
    DataType:    models.DataTypeNumber,
    Description: "Number of active DTCs",
},
```

### 2. **Action-Based Services** (POST operations)
For operations that change vehicle state or perform actions.

**Implementation Pattern:**
1. Create new request/response models in `models/models.go`
2. Add service method in `services/sovd_service.go`
3. Add handler in `handlers/sovd_handler.go`
4. Add route in `main.go`
5. Update OpenAPI spec

**Example:** Actuator Control Service
```go
// 1. Models
type ActuatorControlRequest struct {
    ActuatorID string      `json:"actuator_id"`
    Action     string      `json:"action"`
    Value      interface{} `json:"value,omitempty"`
}

// 2. Service method
func (s *SOVDService) ControlActuator(componentID string, request *ActuatorControlRequest) (*ActuatorControlResponse, error) {
    // Implementation logic
}

// 3. Handler
func (h *SOVDHandler) ControlActuator(c *gin.Context) {
    // HTTP handling logic
}

// 4. Route
v1.POST("/components/:component_id/actuators/control", sovdHandler.ControlActuator)
```

### 3. **Generic Service Framework**
For flexible service execution that can handle multiple service types.

**Use when:**
- You need to support multiple UDS services
- Service types vary by component
- You want maximum flexibility

**Example Implementation:**
```go
type ServiceRequest struct {
    ServiceType string                 `json:"service_type"`
    Parameters  map[string]interface{} `json:"parameters"`
}

func (s *SOVDService) ExecuteService(componentID string, request *ServiceRequest) (*ServiceResponse, error) {
    switch request.ServiceType {
    case "routine":
        return s.executeRoutine(componentID, request.Parameters)
    case "security_access":
        return s.handleSecurityAccess(componentID, request.Parameters)
    // ... other service types
    }
}
```

## Service Categories and UDS Mapping

### Common Service Types and Their UDS Equivalents

| Service Type | UDS Service | Description | HTTP Method |
|--------------|-------------|-------------|-------------|
| **Data Reading** | 0x22 ReadDataByIdentifier | Read sensor values, VIN, etc. | GET |
| **Actuator Control** | 0x2F InputOutputControlByIdentifier | Control actuators, outputs | POST |
| **DTC Management** | 0x19 ReadDTCInformation<br>0x14 ClearDiagnosticInformation | Read/clear trouble codes | POST |
| **Routine Execution** | 0x31 RoutineControl | Execute diagnostic routines | POST |
| **Memory Operations** | 0x23 ReadMemoryByAddress<br>0x3D WriteMemoryByAddress | Read/write ECU memory | GET/POST |
| **Security Access** | 0x27 SecurityAccess | Unlock protected functions | POST |
| **Session Control** | 0x10 DiagnosticSessionControl | Change diagnostic sessions | POST |
| **Tester Present** | 0x3E TesterPresent | Keep session alive | POST |

## Best Practices for Service Implementation

### 1. **Consistent Error Handling**
```go
func (s *SOVDService) YourService(componentID string, request *YourRequest) (*YourResponse, error) {
    // Validate component exists
    if !s.componentExists(componentID) {
        return nil, fmt.Errorf("component '%s' not found", componentID)
    }
    
    // Validate request parameters
    if request.RequiredField == "" {
        return nil, fmt.Errorf("required field missing")
    }
    
    // Execute service logic
    // ...
    
    return response, nil
}
```

### 2. **Structured Response Format**
```go
type StandardResponse struct {
    Status     string                 `json:"status"`      // "success", "failed", "in_progress"
    Message    string                 `json:"message"`     // Human-readable message
    Results    map[string]interface{} `json:"results"`     // Service-specific results
    Timestamp  *time.Time             `json:"timestamp"`   // Operation timestamp
    RequestID  string                 `json:"request_id"`  // For tracking async operations
}
```

### 3. **Component-Specific Logic**
```go
func (s *SOVDService) ControlActuator(componentID string, request *ActuatorControlRequest) (*ActuatorControlResponse, error) {
    // Component-specific actuator validation
    validActuators := map[string][]string{
        "engine": {"fuel_pump", "cooling_fan", "throttle"},
        "bcm":    {"headlights", "horn", "windows"},
        "abs":    {"brake_pressure", "wheel_valves"},
    }
    
    actuators, exists := validActuators[componentID]
    if !exists {
        return nil, fmt.Errorf("no actuators available for component '%s'", componentID)
    }
    
    // Validate actuator exists for this component
    // ...
}
```

### 4. **Future UDS Integration Points**
```go
func (s *SOVDService) ControlActuator(componentID string, request *ActuatorControlRequest) (*ActuatorControlResponse, error) {
    // Current: Mock implementation
    response := &ActuatorControlResponse{
        Status: "success",
        // ... mock data
    }
    
    // Future: Real UDS implementation
    /*
    udsRequest := &uds.IOControlRequest{
        DataIdentifier: s.getActuatorDID(request.ActuatorID),
        ControlOption:  s.getControlOption(request.Action),
        ControlState:   s.encodeValue(request.Value),
    }
    
    udsResponse, err := s.udsClient.SendIOControl(componentID, udsRequest)
    if err != nil {
        return nil, err
    }
    
    response.Status = s.mapUdsStatus(udsResponse.ResponseCode)
    response.Value = s.decodeValue(udsResponse.Data)
    */
    
    return response, nil
}
```

## Adding Async Operations

For long-running operations, implement async patterns:

### 1. **Job Queue Pattern**
```go
type AsyncJob struct {
    ID          string    `json:"id"`
    Status      string    `json:"status"`      // "queued", "running", "completed", "failed"
    Progress    int       `json:"progress"`    // 0-100
    StartTime   time.Time `json:"start_time"`
    CompletedAt *time.Time `json:"completed_at,omitempty"`
    Results     interface{} `json:"results,omitempty"`
    Error       string    `json:"error,omitempty"`
}

// Start async operation
func (h *SOVDHandler) StartAsyncOperation(c *gin.Context) {
    job := &AsyncJob{
        ID:        generateJobID(),
        Status:    "queued",
        StartTime: time.Now(),
    }
    
    // Store job and start processing
    h.jobManager.StartJob(job, func() {
        // Long-running operation
    })
    
    c.JSON(http.StatusAccepted, job)
}

// Check job status
func (h *SOVDHandler) GetJobStatus(c *gin.Context) {
    jobID := c.Param("job_id")
    job := h.jobManager.GetJob(jobID)
    c.JSON(http.StatusOK, job)
}
```

## Testing Your New Services

### 1. **Unit Tests**
```go
func TestControlActuator(t *testing.T) {
    service := services.NewSOVDService()
    
    request := &models.ActuatorControlRequest{
        ActuatorID: "fuel_pump",
        Action:     "start",
    }
    
    response, err := service.ControlActuator("engine", request)
    
    assert.NoError(t, err)
    assert.Equal(t, "success", response.Status)
    assert.Equal(t, "fuel_pump", response.ActuatorID)
}
```

### 2. **Integration Tests**
```go
func TestActuatorControlEndpoint(t *testing.T) {
    router := setupTestRouter()
    
    requestBody := `{
        "actuator_id": "fuel_pump",
        "action": "start"
    }`
    
    req := httptest.NewRequest("POST", "/api/v1/components/engine/actuators/control", strings.NewReader(requestBody))
    req.Header.Set("Content-Type", "application/json")
    
    w := httptest.NewRecorder()
    router.ServeHTTP(w, req)
    
    assert.Equal(t, http.StatusOK, w.Code)
}
```

## Migration to Real UDS

When integrating with your C++ UDS library:

### 1. **Replace Mock Data**
```go
// Current mock
func (s *SOVDService) GetDataItemValue(componentID, dataID string) (*models.DataItemValue, error) {
    mockData := map[string]map[string]*models.DataItemValue{
        // ... mock data
    }
    return mockData[componentID][dataID], nil
}

// With UDS integration
func (s *SOVDService) GetDataItemValue(componentID, dataID string) (*models.DataItemValue, error) {
    did := s.getDataIdentifier(dataID)
    response, err := s.udsClient.ReadDataByIdentifier(componentID, did)
    if err != nil {
        return nil, err
    }
    
    return &models.DataItemValue{
        ID:        dataID,
        Data:      s.parseUdsData(dataID, response.Data),
        Timestamp: timePtr(time.Now()),
        Quality:   s.mapUdsQuality(response.ResponseCode),
    }, nil
}
```

### 2. **Add Component Addressing**
```go
type ComponentConfig struct {
    Address     uint32 `json:"address"`      // UDS address
    Interface   string `json:"interface"`    // "can0", "eth0", etc.
    Timeout     int    `json:"timeout"`      // Request timeout ms
    KeepAlive   bool   `json:"keep_alive"`   // Maintain connection
}

var componentConfigs = map[string]ComponentConfig{
    "engine":       {Address: 0x7E0, Interface: "can0", Timeout: 1000},
    "transmission": {Address: 0x7E1, Interface: "can0", Timeout: 1000},
    "abs":          {Address: 0x7E2, Interface: "can0", Timeout: 1000},
}
```

This architecture allows you to gradually migrate from mock data to real UDS communication while maintaining API compatibility.