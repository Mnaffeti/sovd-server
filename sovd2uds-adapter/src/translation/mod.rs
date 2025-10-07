use crate::error::{Result, Sovd2UdsError};
use crate::models::uds::data_identifiers;
use crate::models::*;
use crate::uds::UdsClient;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, info};

/// Translator between SOVD and UDS protocols
pub struct SovdUdsTranslator {
    /// Mapping of SOVD data item IDs to UDS DIDs
    did_mappings: HashMap<String, u16>,
    /// Mapping of SOVD component categories
    category_mappings: HashMap<String, Vec<String>>,
}

impl SovdUdsTranslator {
    /// Create a new translator with default mappings
    pub fn new() -> Self {
        let mut translator = Self {
            did_mappings: HashMap::new(),
            category_mappings: HashMap::new(),
        };

        translator.initialize_default_mappings();
        translator
    }

    /// Initialize default SOVD to UDS mappings
    fn initialize_default_mappings(&mut self) {
        // Map common SOVD data items to UDS DIDs
        self.did_mappings.insert("vin".to_string(), data_identifiers::VIN);
        self.did_mappings.insert("ecu_serial_number".to_string(), data_identifiers::ECU_SERIAL_NUMBER);
        self.did_mappings.insert("ecu_hardware_version".to_string(), data_identifiers::ECU_HARDWARE_VERSION);
        self.did_mappings.insert("ecu_software_version".to_string(), data_identifiers::ECU_SOFTWARE_VERSION);
        self.did_mappings.insert("manufacturing_date".to_string(), data_identifiers::ECU_MANUFACTURING_DATE);
        self.did_mappings.insert("system_supplier_id".to_string(), data_identifiers::SYSTEM_SUPPLIER_ID);

        // Category mappings
        let mut ident_data = Vec::new();
        ident_data.push("vin".to_string());
        ident_data.push("ecu_serial_number".to_string());
        ident_data.push("ecu_hardware_version".to_string());
        ident_data.push("ecu_software_version".to_string());
        ident_data.push("manufacturing_date".to_string());
        ident_data.push("system_supplier_id".to_string());

        self.category_mappings.insert("identData".to_string(), ident_data);
    }

    /// Add a custom DID mapping
    pub fn add_did_mapping(&mut self, sovd_id: String, did: u16) {
        self.did_mappings.insert(sovd_id, did);
    }

    /// Get UDS DID for a SOVD data item ID
    pub fn get_did(&self, data_item_id: &str) -> Option<u16> {
        self.did_mappings.get(data_item_id).copied()
    }

    /// Get all components (static list for now)
    pub fn get_components(&self) -> Vec<Component> {
        vec![
            Component {
                id: "engine".to_string(),
                name: "Engine Control Unit".to_string(),
                description: Some("Main engine control unit".to_string()),
            },
            Component {
                id: "transmission".to_string(),
                name: "Transmission Control Unit".to_string(),
                description: Some("Automatic transmission control".to_string()),
            },
            Component {
                id: "abs".to_string(),
                name: "ABS Control Unit".to_string(),
                description: Some("Anti-lock braking system".to_string()),
            },
            Component {
                id: "airbag".to_string(),
                name: "Airbag Control Unit".to_string(),
                description: Some("Airbag and restraint system".to_string()),
            },
        ]
    }

    /// Get data items for a component
    pub fn get_component_data_items(&self, categories: Option<&[String]>) -> Vec<DataItem> {
        let mut items = Vec::new();

        // Filter by categories if specified
        let item_ids: Vec<String> = if let Some(cats) = categories {
            cats.iter()
                .filter_map(|cat| self.category_mappings.get(cat))
                .flatten()
                .cloned()
                .collect()
        } else {
            self.did_mappings.keys().cloned().collect()
        };

        for item_id in item_ids {
            items.push(self.create_data_item(&item_id));
        }

        items
    }

    /// Create a DataItem from an ID
    fn create_data_item(&self, item_id: &str) -> DataItem {
        let (name, category, data_type, description) = match item_id {
            "vin" => (
                "Vehicle Identification Number",
                "identData",
                "string",
                "Unique vehicle identification number",
            ),
            "ecu_serial_number" => (
                "ECU Serial Number",
                "identData",
                "string",
                "ECU serial number",
            ),
            "ecu_hardware_version" => (
                "ECU Hardware Version",
                "identData",
                "string",
                "ECU hardware version",
            ),
            "ecu_software_version" => (
                "ECU Software Version",
                "identData",
                "string",
                "ECU software version",
            ),
            "manufacturing_date" => (
                "Manufacturing Date",
                "identData",
                "string",
                "ECU manufacturing date",
            ),
            "system_supplier_id" => (
                "System Supplier ID",
                "identData",
                "string",
                "System supplier identifier",
            ),
            _ => (item_id, "unknown", "string", "Unknown data item"),
        };

        DataItem {
            id: item_id.to_string(),
            name: name.to_string(),
            category: category.to_string(),
            data_type: Some(data_type.to_string()),
            description: Some(description.to_string()),
        }
    }

    /// Read a data item value from UDS
    pub async fn read_data_item(
        &self,
        client: &UdsClient,
        data_item_id: &str,
    ) -> Result<DataItemValue> {
        debug!("Reading data item: {}", data_item_id);

        // Get DID for the data item
        let did = self
            .get_did(data_item_id)
            .ok_or_else(|| Sovd2UdsError::DataItemNotFound(data_item_id.to_string()))?;

        // Read from UDS
        let raw_data = client.read_data_by_identifier(did).await?;

        // Convert to appropriate format
        let data_item = self.create_data_item(data_item_id);
        let data_value = self.convert_uds_data_to_sovd(&raw_data, &data_item.data_type.unwrap_or_else(|| "string".to_string()))?;

        Ok(DataItemValue {
            id: data_item_id.to_string(),
            name: data_item.name,
            category: data_item.category,
            data: data_value,
            timestamp: Some(Utc::now()),
            quality: Some("good".to_string()),
        })
    }

    /// Convert UDS raw data to SOVD JSON value
    fn convert_uds_data_to_sovd(&self, data: &[u8], data_type: &str) -> Result<serde_json::Value> {
        match data_type {
            "string" => {
                let s = String::from_utf8(data.to_vec())
                    .unwrap_or_else(|_| hex::encode(data));
                Ok(serde_json::Value::String(s))
            }
            "number" => {
                if data.len() == 1 {
                    Ok(serde_json::Value::Number(data[0].into()))
                } else if data.len() == 2 {
                    let value = u16::from_be_bytes([data[0], data[1]]);
                    Ok(serde_json::Value::Number(value.into()))
                } else if data.len() == 4 {
                    let value = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
                    Ok(serde_json::Value::Number(value.into()))
                } else {
                    Ok(serde_json::Value::String(hex::encode(data)))
                }
            }
            "boolean" => {
                Ok(serde_json::Value::Bool(data.first().map(|&b| b != 0).unwrap_or(false)))
            }
            _ => {
                // Default: hex string
                Ok(serde_json::Value::String(hex::encode(data)))
            }
        }
    }

    /// Translate DTC management request to UDS operations
    pub async fn manage_dtcs(
        &self,
        client: &UdsClient,
        request: &DtcManagementRequest,
    ) -> Result<DtcManagementResponse> {
        match request.action.as_str() {
            "clear" => {
                // Clear all DTCs (group 0xFFFFFF)
                client.clear_diagnostic_information(0xFFFFFF).await?;
                
                Ok(DtcManagementResponse {
                    action: "clear".to_string(),
                    status: "success".to_string(),
                    results: None,
                    message: Some("All DTCs cleared successfully".to_string()),
                    timestamp: Some(Utc::now()),
                })
            }
            "read" => {
                // Read DTCs with status mask (sub-function 0x02: report DTC by status mask)
                let dtc_data = client.read_dtc_information(0x02).await?;
                
                // Parse DTC data
                let dtcs = self.parse_dtc_data(&dtc_data)?;
                
                Ok(DtcManagementResponse {
                    action: "read".to_string(),
                    status: "success".to_string(),
                    results: Some(serde_json::json!({ "dtcs": dtcs })),
                    message: Some(format!("Found {} DTCs", dtcs.len())),
                    timestamp: Some(Utc::now()),
                })
            }
            "freeze_frame" => {
                // Read freeze frame data (sub-function 0x04)
                let freeze_frame_data = client.read_dtc_information(0x04).await?;
                
                Ok(DtcManagementResponse {
                    action: "freeze_frame".to_string(),
                    status: "success".to_string(),
                    results: Some(serde_json::json!({ 
                        "freeze_frame_data": hex::encode(&freeze_frame_data) 
                    })),
                    message: Some("Freeze frame data retrieved".to_string()),
                    timestamp: Some(Utc::now()),
                })
            }
            _ => {
                Err(Sovd2UdsError::InvalidRequest(format!(
                    "Unknown DTC action: {}",
                    request.action
                )))
            }
        }
    }

    /// Parse DTC data from UDS response
    fn parse_dtc_data(&self, data: &[u8]) -> Result<Vec<serde_json::Value>> {
        let mut dtcs = Vec::new();
        
        // Skip status availability mask (first byte)
        if data.len() < 1 {
            return Ok(dtcs);
        }

        let mut offset = 1;
        
        // Each DTC entry is 4 bytes: 3 bytes DTC + 1 byte status
        while offset + 4 <= data.len() {
            let dtc_bytes = &data[offset..offset + 3];
            let status = data[offset + 3];
            
            // Convert DTC bytes to standard format (e.g., P0100)
            let dtc_string = self.format_dtc(dtc_bytes);
            
            dtcs.push(serde_json::json!({
                "code": dtc_string,
                "status": format!("0x{:02X}", status),
                "description": self.get_dtc_description(&dtc_string),
            }));
            
            offset += 4;
        }

        Ok(dtcs)
    }

    /// Format DTC bytes into standard string format
    fn format_dtc(&self, bytes: &[u8]) -> String {
        if bytes.len() < 3 {
            return "UNKNOWN".to_string();
        }

        let first_byte = bytes[0];
        let prefix = match (first_byte >> 6) & 0x03 {
            0 => 'P', // Powertrain
            1 => 'C', // Chassis
            2 => 'B', // Body
            3 => 'U', // Network
            _ => 'X',
        };

        let second_digit = (first_byte >> 4) & 0x03;
        let third_digit = first_byte & 0x0F;
        let fourth_fifth = bytes[1];

        format!(
            "{}{}{}{}{}",
            prefix,
            second_digit,
            third_digit,
            (fourth_fifth >> 4) & 0x0F,
            fourth_fifth & 0x0F
        )
    }

    /// Get DTC description (placeholder)
    fn get_dtc_description(&self, _dtc: &str) -> String {
        "Diagnostic trouble code".to_string()
    }

    /// Execute a routine (actuator control)
    pub async fn control_actuator(
        &self,
        client: &UdsClient,
        request: &ActuatorControlRequest,
    ) -> Result<ActuatorControlResponse> {
        // Map actuator ID to routine ID (this is application-specific)
        let routine_id = self.get_routine_id(&request.actuator_id)?;

        let control_type = match request.action.as_str() {
            "start" => RoutineControlType::StartRoutine,
            "stop" => RoutineControlType::StopRoutine,
            _ => {
                return Err(Sovd2UdsError::InvalidRequest(format!(
                    "Unknown actuator action: {}",
                    request.action
                )));
            }
        };

        // Prepare routine parameters
        let params = if let Some(value) = &request.value {
            self.serialize_value_to_bytes(value)?
        } else {
            vec![]
        };

        let _response = client.routine_control(control_type, routine_id, &params).await?;

        Ok(ActuatorControlResponse {
            actuator_id: request.actuator_id.clone(),
            action: request.action.clone(),
            status: "success".to_string(),
            value: request.value.clone(),
            message: Some(format!("Actuator {} {} successfully", request.actuator_id, request.action)),
            timestamp: Some(Utc::now()),
        })
    }

    /// Get routine ID for an actuator
    fn get_routine_id(&self, actuator_id: &str) -> Result<u16> {
        // This is application-specific mapping
        // Add your actuator to routine ID mappings here
        match actuator_id {
            "fuel_pump" => Ok(0x0201),
            "cooling_fan" => Ok(0x0202),
            "throttle" => Ok(0x0203),
            _ => Err(Sovd2UdsError::InvalidRequest(format!(
                "Unknown actuator: {}",
                actuator_id
            ))),
        }
    }

    /// Serialize JSON value to bytes
    fn serialize_value_to_bytes(&self, value: &serde_json::Value) -> Result<Vec<u8>> {
        match value {
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_u64() {
                    if i <= 255 {
                        Ok(vec![i as u8])
                    } else if i <= 65535 {
                        Ok((i as u16).to_be_bytes().to_vec())
                    } else {
                        Ok((i as u32).to_be_bytes().to_vec())
                    }
                } else {
                    Err(Sovd2UdsError::InvalidRequest("Invalid number value".to_string()))
                }
            }
            serde_json::Value::Bool(b) => Ok(vec![if *b { 1 } else { 0 }]),
            serde_json::Value::String(s) => Ok(s.as_bytes().to_vec()),
            _ => Err(Sovd2UdsError::InvalidRequest("Unsupported value type".to_string())),
        }
    }
}

impl Default for SovdUdsTranslator {
    fn default() -> Self {
        Self::new()
    }
}

// Helper module for hex encoding
mod hex {
    pub fn encode(data: &[u8]) -> String {
        data.iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<_>>()
            .join("")
    }
}
