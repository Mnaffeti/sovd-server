use crate::config::Config;
use crate::error::{Result, Sovd2UdsError, UdsNegativeResponseCode};
use crate::ffi::UdsClientHandle;
use crate::models::uds::*;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

/// High-level UDS client abstraction
pub struct UdsClient {
    handle: Arc<RwLock<UdsClientHandle>>,
    config: Arc<Config>,
    component_id: String,
    ecu_address: u32,
}

impl UdsClient {
    /// Create a new UDS client for a specific component
    pub fn new(config: Arc<Config>, component_id: String) -> Result<Self> {
        let ecu_address = config
            .get_component_address(&component_id)
            .ok_or_else(|| Sovd2UdsError::ComponentNotFound(component_id.clone()))?;

        let handle = UdsClientHandle::new(
            &config.uds.interface,
            ecu_address,
            config.uds.timeout,
        )?;

        Ok(Self {
            handle: Arc::new(RwLock::new(handle)),
            config,
            component_id,
            ecu_address,
        })
    }

    /// Connect to the ECU
    pub async fn connect(&self) -> Result<()> {
        let handle = self.handle.read().await;
        handle.connect()?;
        info!(
            "Connected to ECU for component '{}' at address 0x{:X}",
            self.component_id, self.ecu_address
        );
        Ok(())
    }

    /// Disconnect from the ECU
    pub async fn disconnect(&self) -> Result<()> {
        let handle = self.handle.read().await;
        handle.disconnect()?;
        info!(
            "Disconnected from ECU for component '{}'",
            self.component_id
        );
        Ok(())
    }

    /// Read data by identifier
    pub async fn read_data_by_identifier(&self, did: u16) -> Result<Vec<u8>> {
        debug!(
            "Reading DID 0x{:04X} from component '{}'",
            did, self.component_id
        );

        let handle = self.handle.read().await;
        let data = handle.read_data_by_identifier(did)?;

        debug!("Successfully read {} bytes from DID 0x{:04X}", data.len(), did);
        Ok(data)
    }

    /// Write data by identifier
    pub async fn write_data_by_identifier(&self, did: u16, data: &[u8]) -> Result<()> {
        debug!(
            "Writing {} bytes to DID 0x{:04X} on component '{}'",
            data.len(),
            did,
            self.component_id
        );

        // Check if security access is required
        if self.config.security.require_security_access {
            self.perform_security_access().await?;
        }

        let handle = self.handle.read().await;
        handle.write_data_by_identifier(did, data)?;

        info!("Successfully wrote to DID 0x{:04X}", did);
        Ok(())
    }

    /// Change diagnostic session
    pub async fn diagnostic_session_control(
        &self,
        session_type: DiagnosticSessionType,
    ) -> Result<Vec<u8>> {
        debug!(
            "Changing to diagnostic session {:?} for component '{}'",
            session_type, self.component_id
        );

        let handle = self.handle.read().await;
        let response = handle.diagnostic_session_control(session_type as u8)?;

        info!("Successfully changed to diagnostic session {:?}", session_type);
        Ok(response)
    }

    /// Reset ECU
    pub async fn ecu_reset(&self, reset_type: EcuResetType) -> Result<Vec<u8>> {
        warn!(
            "Performing ECU reset {:?} for component '{}'",
            reset_type, self.component_id
        );

        let handle = self.handle.read().await;
        let response = handle.ecu_reset(reset_type as u8)?;

        warn!("ECU reset {:?} executed", reset_type);
        Ok(response)
    }

    /// Perform security access (request seed and send key)
    async fn perform_security_access(&self) -> Result<()> {
        debug!("Performing security access");

        let security_level = self.config.security.security_level;
        
        // Request seed (odd sub-function)
        let request_seed_type = security_level * 2 - 1;
        
        let handle = self.handle.read().await;
        let seed = handle.security_access(request_seed_type, &[])?;

        if seed.is_empty() {
            info!("Security access already granted");
            return Ok(());
        }

        // Calculate key from seed (this is application-specific)
        // For now, we'll use a placeholder implementation
        let key = self.calculate_security_key(&seed);

        // Send key (even sub-function)
        let send_key_type = security_level * 2;
        handle.security_access(send_key_type, &key)?;

        info!("Security access granted");
        Ok(())
    }

    /// Calculate security key from seed (placeholder implementation)
    /// In a real implementation, this would use the actual security algorithm
    fn calculate_security_key(&self, seed: &[u8]) -> Vec<u8> {
        // Placeholder: XOR with a constant
        // Replace with actual algorithm
        seed.iter().map(|b| b ^ 0xAA).collect()
    }

    /// Read DTC information
    pub async fn read_dtc_information(&self, sub_function: u8) -> Result<Vec<u8>> {
        debug!(
            "Reading DTC information (sub-function 0x{:02X}) from component '{}'",
            sub_function, self.component_id
        );

        let handle = self.handle.read().await;
        let data = handle.read_dtc_information(sub_function)?;

        debug!("Successfully read DTC information");
        Ok(data)
    }

    /// Clear diagnostic information
    pub async fn clear_diagnostic_information(&self, group: u32) -> Result<()> {
        info!(
            "Clearing diagnostic information (group 0x{:06X}) for component '{}'",
            group, self.component_id
        );

        let handle = self.handle.read().await;
        handle.clear_diagnostic_information(group)?;

        info!("Successfully cleared diagnostic information");
        Ok(())
    }

    /// Control routine
    pub async fn routine_control(
        &self,
        control_type: RoutineControlType,
        routine_id: u16,
        params: &[u8],
    ) -> Result<Vec<u8>> {
        debug!(
            "Routine control {:?} for routine 0x{:04X} on component '{}'",
            control_type, routine_id, self.component_id
        );

        let handle = self.handle.read().await;
        let response = handle.routine_control(control_type as u8, routine_id, params)?;

        info!("Routine control {:?} executed successfully", control_type);
        Ok(response)
    }

    /// Get VIN (Vehicle Identification Number)
    pub async fn get_vin(&self) -> Result<String> {
        let data = self
            .read_data_by_identifier(data_identifiers::VIN)
            .await?;
        
        String::from_utf8(data)
            .map_err(|e| Sovd2UdsError::Translation(format!("Invalid VIN data: {}", e)))
    }

    /// Get ECU software version
    pub async fn get_software_version(&self) -> Result<String> {
        let data = self
            .read_data_by_identifier(data_identifiers::ECU_SOFTWARE_VERSION)
            .await?;
        
        String::from_utf8(data)
            .map_err(|e| Sovd2UdsError::Translation(format!("Invalid software version data: {}", e)))
    }

    /// Get ECU hardware version
    pub async fn get_hardware_version(&self) -> Result<String> {
        let data = self
            .read_data_by_identifier(data_identifiers::ECU_HARDWARE_VERSION)
            .await?;
        
        String::from_utf8(data)
            .map_err(|e| Sovd2UdsError::Translation(format!("Invalid hardware version data: {}", e)))
    }

    /// Get component ID
    pub fn component_id(&self) -> &str {
        &self.component_id
    }

    /// Get ECU address
    pub fn ecu_address(&self) -> u32 {
        self.ecu_address
    }
}

/// UDS Client pool for managing multiple connections
pub struct UdsClientPool {
    config: Arc<Config>,
    clients: Arc<RwLock<std::collections::HashMap<String, Arc<UdsClient>>>>,
}

impl UdsClientPool {
    /// Create a new UDS client pool
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            clients: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// Get or create a UDS client for a component
    pub async fn get_client(&self, component_id: &str) -> Result<Arc<UdsClient>> {
        let mut clients = self.clients.write().await;

        if let Some(client) = clients.get(component_id) {
            return Ok(Arc::clone(client));
        }

        // Create new client
        let client = Arc::new(UdsClient::new(
            Arc::clone(&self.config),
            component_id.to_string(),
        )?);

        // Connect to ECU
        client.connect().await?;

        clients.insert(component_id.to_string(), Arc::clone(&client));

        Ok(client)
    }

    /// Remove a client from the pool
    pub async fn remove_client(&self, component_id: &str) -> Result<()> {
        let mut clients = self.clients.write().await;
        
        if let Some(client) = clients.remove(component_id) {
            client.disconnect().await?;
        }

        Ok(())
    }

    /// Close all connections
    pub async fn close_all(&self) -> Result<()> {
        let mut clients = self.clients.write().await;
        
        for (_, client) in clients.drain() {
            if let Err(e) = client.disconnect().await {
                error!("Failed to disconnect client: {}", e);
            }
        }

        Ok(())
    }
}
