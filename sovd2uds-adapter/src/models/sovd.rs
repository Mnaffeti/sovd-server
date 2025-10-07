use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// SOVD Component representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// SOVD Components response
#[derive(Debug, Serialize, Deserialize)]
pub struct ComponentsResponse {
    pub components: Vec<Component>,
}

/// SOVD Data Item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataItem {
    pub id: String,
    pub name: String,
    pub category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// SOVD Data Items response
#[derive(Debug, Serialize, Deserialize)]
pub struct DataItemsResponse {
    pub items: Vec<DataItem>,
}

/// SOVD Data Item Value
#[derive(Debug, Serialize, Deserialize)]
pub struct DataItemValue {
    pub id: String,
    pub name: String,
    pub category: String,
    pub data: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<String>,
}

/// SOVD Actuator Control Request
#[derive(Debug, Deserialize)]
pub struct ActuatorControlRequest {
    pub actuator_id: String,
    pub action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,
}

/// SOVD Actuator Control Response
#[derive(Debug, Serialize)]
pub struct ActuatorControlResponse {
    pub actuator_id: String,
    pub action: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,
}

/// SOVD DTC Management Request
#[derive(Debug, Deserialize)]
pub struct DtcManagementRequest {
    pub action: String, // "clear", "read", "freeze_frame"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dtcs: Option<Vec<String>>,
}

/// SOVD DTC Management Response
#[derive(Debug, Serialize)]
pub struct DtcManagementResponse {
    pub action: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,
}

/// SOVD Service Request
#[derive(Debug, Deserialize)]
pub struct ServiceRequest {
    pub service_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, serde_json::Value>>,
}

/// SOVD Service Response
#[derive(Debug, Serialize)]
pub struct ServiceResponse {
    pub service_type: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,
}

/// SOVD Error Response
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}
