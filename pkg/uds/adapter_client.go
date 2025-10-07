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
	baseURL    string
	httpClient *http.Client
}

// NewAdapterClient creates a new adapter client
func NewAdapterClient(baseURL string) *AdapterClient {
	return &AdapterClient{
		baseURL: baseURL,
		httpClient: &http.Client{
			Timeout: 30 * time.Second,
		},
	}
}

// DataItemValue represents a data item response from the adapter
type DataItemValue struct {
	ID        string      `json:"id"`
	Name      string      `json:"name"`
	Category  string      `json:"category"`
	Data      interface{} `json:"data"`
	Timestamp string      `json:"timestamp,omitempty"`
	Quality   string      `json:"quality,omitempty"`
}

// DataItem represents a data item definition
type DataItem struct {
	ID          string `json:"id"`
	Name        string `json:"name"`
	Category    string `json:"category"`
	DataType    string `json:"data_type,omitempty"`
	Description string `json:"description,omitempty"`
}

// DataItemsResponse represents the response containing multiple data items
type DataItemsResponse struct {
	Items []DataItem `json:"items"`
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

// ActuatorControlRequest represents an actuator control request
type ActuatorControlRequest struct {
	ActuatorID string      `json:"actuator_id"`
	Action     string      `json:"action"`
	Value      interface{} `json:"value,omitempty"`
	Duration   int         `json:"duration,omitempty"`
}

// ActuatorControlResponse represents an actuator control response
type ActuatorControlResponse struct {
	ActuatorID string      `json:"actuator_id"`
	Action     string      `json:"action"`
	Status     string      `json:"status"`
	Value      interface{} `json:"value,omitempty"`
	Message    string      `json:"message,omitempty"`
	Timestamp  string      `json:"timestamp,omitempty"`
}

// ServiceRequest represents a generic service request
type ServiceRequest struct {
	ServiceType string                 `json:"service_type"`
	Parameters  map[string]interface{} `json:"parameters,omitempty"`
}

// ServiceResponse represents a generic service response
type ServiceResponse struct {
	ServiceType string      `json:"service_type"`
	Status      string      `json:"status"`
	Results     interface{} `json:"results,omitempty"`
	Message     string      `json:"message,omitempty"`
	Timestamp   string      `json:"timestamp,omitempty"`
}

// ErrorResponse represents an error response from the adapter
type ErrorResponse struct {
	Error   string `json:"error"`
	Code    int    `json:"code"`
	Details string `json:"details,omitempty"`
}

// ReadDataItem reads a data item from a component
func (c *AdapterClient) ReadDataItem(componentID, dataID string) (*DataItemValue, error) {
	url := fmt.Sprintf("%s/api/v1/components/%s/data/%s", c.baseURL, componentID, dataID)

	resp, err := c.httpClient.Get(url)
	if err != nil {
		return nil, fmt.Errorf("failed to call adapter: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		var errResp ErrorResponse
		if err := json.NewDecoder(resp.Body).Decode(&errResp); err != nil {
			body, _ := io.ReadAll(resp.Body)
			return nil, fmt.Errorf("adapter error (status %d): %s", resp.StatusCode, string(body))
		}
		return nil, fmt.Errorf("adapter error: %s", errResp.Error)
	}

	var result DataItemValue
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		return nil, fmt.Errorf("failed to decode response: %w", err)
	}

	return &result, nil
}

// GetComponentDataItems retrieves available data items for a component
func (c *AdapterClient) GetComponentDataItems(componentID string, categories []string) (*DataItemsResponse, error) {
	url := fmt.Sprintf("%s/api/v1/components/%s/data", c.baseURL, componentID)

	// Add categories as query parameter if provided
	if len(categories) > 0 {
		categoriesStr := ""
		for i, cat := range categories {
			if i > 0 {
				categoriesStr += ","
			}
			categoriesStr += cat
		}
		url += "?categories=" + categoriesStr
	}

	resp, err := c.httpClient.Get(url)
	if err != nil {
		return nil, fmt.Errorf("failed to call adapter: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		var errResp ErrorResponse
		if err := json.NewDecoder(resp.Body).Decode(&errResp); err != nil {
			body, _ := io.ReadAll(resp.Body)
			return nil, fmt.Errorf("adapter error (status %d): %s", resp.StatusCode, string(body))
		}
		return nil, fmt.Errorf("adapter error: %s", errResp.Error)
	}

	var result DataItemsResponse
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		return nil, fmt.Errorf("failed to decode response: %w", err)
	}

	return &result, nil
}

// ManageDTCs performs DTC operations (read, clear, freeze_frame)
func (c *AdapterClient) ManageDTCs(componentID string, request *DTCManagementRequest) (*DTCManagementResponse, error) {
	url := fmt.Sprintf("%s/api/v1/components/%s/dtcs", c.baseURL, componentID)

	payload, err := json.Marshal(request)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal request: %w", err)
	}

	resp, err := c.httpClient.Post(url, "application/json", bytes.NewBuffer(payload))
	if err != nil {
		return nil, fmt.Errorf("failed to call adapter: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		var errResp ErrorResponse
		if err := json.NewDecoder(resp.Body).Decode(&errResp); err != nil {
			body, _ := io.ReadAll(resp.Body)
			return nil, fmt.Errorf("adapter error (status %d): %s", resp.StatusCode, string(body))
		}
		return nil, fmt.Errorf("adapter error: %s", errResp.Error)
	}

	var result DTCManagementResponse
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		return nil, fmt.Errorf("failed to decode response: %w", err)
	}

	return &result, nil
}

// ControlActuator controls an actuator on a component
func (c *AdapterClient) ControlActuator(componentID string, request *ActuatorControlRequest) (*ActuatorControlResponse, error) {
	url := fmt.Sprintf("%s/api/v1/components/%s/actuators/control", c.baseURL, componentID)

	payload, err := json.Marshal(request)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal request: %w", err)
	}

	resp, err := c.httpClient.Post(url, "application/json", bytes.NewBuffer(payload))
	if err != nil {
		return nil, fmt.Errorf("failed to call adapter: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		var errResp ErrorResponse
		if err := json.NewDecoder(resp.Body).Decode(&errResp); err != nil {
			body, _ := io.ReadAll(resp.Body)
			return nil, fmt.Errorf("adapter error (status %d): %s", resp.StatusCode, string(body))
		}
		return nil, fmt.Errorf("adapter error: %s", errResp.Error)
	}

	var result ActuatorControlResponse
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		return nil, fmt.Errorf("failed to decode response: %w", err)
	}

	return &result, nil
}

// ExecuteService executes a generic service on a component
func (c *AdapterClient) ExecuteService(componentID string, request *ServiceRequest) (*ServiceResponse, error) {
	url := fmt.Sprintf("%s/api/v1/components/%s/services", c.baseURL, componentID)

	payload, err := json.Marshal(request)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal request: %w", err)
	}

	resp, err := c.httpClient.Post(url, "application/json", bytes.NewBuffer(payload))
	if err != nil {
		return nil, fmt.Errorf("failed to call adapter: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		var errResp ErrorResponse
		if err := json.NewDecoder(resp.Body).Decode(&errResp); err != nil {
			body, _ := io.ReadAll(resp.Body)
			return nil, fmt.Errorf("adapter error (status %d): %s", resp.StatusCode, string(body))
		}
		return nil, fmt.Errorf("adapter error: %s", errResp.Error)
	}

	var result ServiceResponse
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		return nil, fmt.Errorf("failed to decode response: %w", err)
	}

	return &result, nil
}

// Health checks if the adapter is running and responsive
func (c *AdapterClient) Health() error {
	resp, err := c.httpClient.Get(c.baseURL + "/health")
	if err != nil {
		return fmt.Errorf("adapter unreachable: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return fmt.Errorf("adapter unhealthy: status %d", resp.StatusCode)
	}

	return nil
}

// SetTimeout sets the HTTP client timeout
func (c *AdapterClient) SetTimeout(timeout time.Duration) {
	c.httpClient.Timeout = timeout
}
