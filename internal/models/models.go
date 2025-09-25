package models

import "time"

// Component represents a vehicle component
type Component struct {
	ID          string `json:"id" binding:"required"`
	Name        string `json:"name" binding:"required"`
	Description string `json:"description,omitempty"`
}

// DataItem represents a data item available in a component
type DataItem struct {
	ID          string `json:"id" binding:"required"`
	Name        string `json:"name" binding:"required"`
	Category    string `json:"category" binding:"required"`
	DataType    string `json:"dataType,omitempty"`
	Description string `json:"description,omitempty"`
}

// DataItemValue represents the actual value of a data item
type DataItemValue struct {
	ID        string      `json:"id" binding:"required"`
	Name      string      `json:"name" binding:"required"`
	Category  string      `json:"category" binding:"required"`
	Data      interface{} `json:"data" binding:"required"`
	Timestamp *time.Time  `json:"timestamp,omitempty"`
	Quality   string      `json:"quality,omitempty"`
}

// ComponentsResponse represents the response for getting all components
type ComponentsResponse struct {
	Components []Component `json:"components"`
}

// DataItemsResponse represents the response for getting component data items
type DataItemsResponse struct {
	Items []DataItem `json:"items"`
}

// ErrorResponse represents an error response
type ErrorResponse struct {
	Error   string `json:"error" binding:"required"`
	Code    int    `json:"code" binding:"required"`
	Details string `json:"details,omitempty"`
}

// DataQuality constants
const (
	QualityGood      = "good"
	QualityBad       = "bad"
	QualityUncertain = "uncertain"
)

// DataTypes constants
const (
	DataTypeString  = "string"
	DataTypeNumber  = "number"
	DataTypeBoolean = "boolean"
	DataTypeArray   = "array"
	DataTypeObject  = "object"
)

// ActuatorControlRequest represents a request to control an actuator
type ActuatorControlRequest struct {
	ActuatorID string      `json:"actuator_id" binding:"required"`
	Action     string      `json:"action" binding:"required"` // "start", "stop", "set_value"
	Value      interface{} `json:"value,omitempty"`           // For set_value actions
	Duration   *int        `json:"duration,omitempty"`        // Duration in seconds
}

// ActuatorControlResponse represents the response to an actuator control request
type ActuatorControlResponse struct {
	ActuatorID string      `json:"actuator_id"`
	Action     string      `json:"action"`
	Status     string      `json:"status"`     // "success", "failed", "in_progress"
	Value      interface{} `json:"value,omitempty"`
	Message    string      `json:"message,omitempty"`
	Timestamp  *time.Time  `json:"timestamp,omitempty"`
}

// DTCManagementRequest represents a DTC management request
type DTCManagementRequest struct {
	Action string   `json:"action" binding:"required"` // "clear", "read", "freeze_frame"
	DTCs   []string `json:"dtcs,omitempty"`            // Specific DTCs to target
}

// DTCManagementResponse represents the response to a DTC management request
type DTCManagementResponse struct {
	Action    string                 `json:"action"`
	Status    string                 `json:"status"`
	Results   map[string]interface{} `json:"results,omitempty"`
	Message   string                 `json:"message,omitempty"`
	Timestamp *time.Time             `json:"timestamp,omitempty"`
}

// ServiceRequest represents a generic service request
type ServiceRequest struct {
	ServiceType string                 `json:"service_type" binding:"required"` // "actuator_control", "dtc_management", "routine"
	Parameters  map[string]interface{} `json:"parameters"`
}

// ServiceResponse represents a generic service response
type ServiceResponse struct {
	ServiceType string                 `json:"service_type"`
	Status      string                 `json:"status"`
	Results     map[string]interface{} `json:"results,omitempty"`
	Message     string                 `json:"message,omitempty"`
	Timestamp   *time.Time             `json:"timestamp,omitempty"`
}