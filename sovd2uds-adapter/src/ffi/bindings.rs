// Include the generated bindings from build.rs
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use crate::error::{Result, Sovd2UdsError};
use std::ffi::{CStr, CString};
use std::ptr;

/// Safe wrapper around the raw UDS client pointer
pub struct UdsClientHandle {
    client: *mut uds_client_t,
}

impl UdsClientHandle {
    /// Create a new UDS client
    pub fn new(interface: &str, address: u32, timeout: u32) -> Result<Self> {
        let interface_c = CString::new(interface)
            .map_err(|e| Sovd2UdsError::Ffi(format!("Invalid interface string: {}", e)))?;

        unsafe {
            let client = uds_client_create(interface_c.as_ptr(), address, timeout);
            if client.is_null() {
                return Err(Sovd2UdsError::Ffi(
                    "Failed to create UDS client".to_string(),
                ));
            }
            Ok(Self { client })
        }
    }

    /// Connect to the UDS target
    pub fn connect(&self) -> Result<()> {
        unsafe {
            let result = uds_client_connect(self.client);
            if result != 0 {
                return Err(Sovd2UdsError::UdsCommunication(format!(
                    "Connection failed with error code: {}",
                    result
                )));
            }
            Ok(())
        }
    }

    /// Disconnect from the UDS target
    pub fn disconnect(&self) -> Result<()> {
        unsafe {
            let result = uds_client_disconnect(self.client);
            if result != 0 {
                return Err(Sovd2UdsError::UdsCommunication(format!(
                    "Disconnection failed with error code: {}",
                    result
                )));
            }
            Ok(())
        }
    }

    /// Read data by identifier
    pub fn read_data_by_identifier(&self, did: u16) -> Result<Vec<u8>> {
        unsafe {
            let response = uds_read_data_by_identifier(self.client, did);
            if response.is_null() {
                return Err(Sovd2UdsError::UdsCommunication(
                    "Null response received".to_string(),
                ));
            }

            let response_ref = &*response;
            
            if response_ref.error_code != 0 {
                let error = Sovd2UdsError::UdsCommunication(format!(
                    "UDS error code: {}",
                    response_ref.error_code
                ));
                uds_response_free(response);
                return Err(error);
            }

            let data = if !response_ref.data.is_null() && response_ref.data_length > 0 {
                std::slice::from_raw_parts(response_ref.data, response_ref.data_length as usize)
                    .to_vec()
            } else {
                vec![]
            };

            uds_response_free(response);
            Ok(data)
        }
    }

    /// Write data by identifier
    pub fn write_data_by_identifier(&self, did: u16, data: &[u8]) -> Result<()> {
        unsafe {
            let response = uds_write_data_by_identifier(
                self.client,
                did,
                data.as_ptr(),
                data.len() as u32,
            );
            
            if response.is_null() {
                return Err(Sovd2UdsError::UdsCommunication(
                    "Null response received".to_string(),
                ));
            }

            let response_ref = &*response;
            let error_code = response_ref.error_code;
            uds_response_free(response);

            if error_code != 0 {
                return Err(Sovd2UdsError::UdsCommunication(format!(
                    "Write failed with error code: {}",
                    error_code
                )));
            }

            Ok(())
        }
    }

    /// Diagnostic session control
    pub fn diagnostic_session_control(&self, session_type: u8) -> Result<Vec<u8>> {
        unsafe {
            let response = uds_diagnostic_session_control(self.client, session_type);
            if response.is_null() {
                return Err(Sovd2UdsError::UdsCommunication(
                    "Null response received".to_string(),
                ));
            }

            let response_ref = &*response;
            
            if response_ref.error_code != 0 {
                let error = Sovd2UdsError::UdsCommunication(format!(
                    "Session control failed with error code: {}",
                    response_ref.error_code
                ));
                uds_response_free(response);
                return Err(error);
            }

            let data = if !response_ref.data.is_null() && response_ref.data_length > 0 {
                std::slice::from_raw_parts(response_ref.data, response_ref.data_length as usize)
                    .to_vec()
            } else {
                vec![]
            };

            uds_response_free(response);
            Ok(data)
        }
    }

    /// ECU reset
    pub fn ecu_reset(&self, reset_type: u8) -> Result<Vec<u8>> {
        unsafe {
            let response = uds_ecu_reset(self.client, reset_type);
            if response.is_null() {
                return Err(Sovd2UdsError::UdsCommunication(
                    "Null response received".to_string(),
                ));
            }

            let response_ref = &*response;
            
            if response_ref.error_code != 0 {
                let error = Sovd2UdsError::UdsCommunication(format!(
                    "ECU reset failed with error code: {}",
                    response_ref.error_code
                ));
                uds_response_free(response);
                return Err(error);
            }

            let data = if !response_ref.data.is_null() && response_ref.data_length > 0 {
                std::slice::from_raw_parts(response_ref.data, response_ref.data_length as usize)
                    .to_vec()
            } else {
                vec![]
            };

            uds_response_free(response);
            Ok(data)
        }
    }

    /// Security access
    pub fn security_access(&self, access_type: u8, key: &[u8]) -> Result<Vec<u8>> {
        unsafe {
            let response = uds_security_access(
                self.client,
                access_type,
                key.as_ptr(),
                key.len() as u32,
            );
            
            if response.is_null() {
                return Err(Sovd2UdsError::UdsCommunication(
                    "Null response received".to_string(),
                ));
            }

            let response_ref = &*response;
            
            if response_ref.error_code != 0 {
                let error = Sovd2UdsError::UdsCommunication(format!(
                    "Security access failed with error code: {}",
                    response_ref.error_code
                ));
                uds_response_free(response);
                return Err(error);
            }

            let data = if !response_ref.data.is_null() && response_ref.data_length > 0 {
                std::slice::from_raw_parts(response_ref.data, response_ref.data_length as usize)
                    .to_vec()
            } else {
                vec![]
            };

            uds_response_free(response);
            Ok(data)
        }
    }

    /// Read DTC information
    pub fn read_dtc_information(&self, sub_function: u8) -> Result<Vec<u8>> {
        unsafe {
            let response = uds_read_dtc_information(self.client, sub_function);
            if response.is_null() {
                return Err(Sovd2UdsError::UdsCommunication(
                    "Null response received".to_string(),
                ));
            }

            let response_ref = &*response;
            
            if response_ref.error_code != 0 {
                let error = Sovd2UdsError::UdsCommunication(format!(
                    "Read DTC failed with error code: {}",
                    response_ref.error_code
                ));
                uds_response_free(response);
                return Err(error);
            }

            let data = if !response_ref.data.is_null() && response_ref.data_length > 0 {
                std::slice::from_raw_parts(response_ref.data, response_ref.data_length as usize)
                    .to_vec()
            } else {
                vec![]
            };

            uds_response_free(response);
            Ok(data)
        }
    }

    /// Clear diagnostic information
    pub fn clear_diagnostic_information(&self, group: u32) -> Result<()> {
        unsafe {
            let response = uds_clear_diagnostic_information(self.client, group);
            if response.is_null() {
                return Err(Sovd2UdsError::UdsCommunication(
                    "Null response received".to_string(),
                ));
            }

            let response_ref = &*response;
            let error_code = response_ref.error_code;
            uds_response_free(response);

            if error_code != 0 {
                return Err(Sovd2UdsError::UdsCommunication(format!(
                    "Clear DTC failed with error code: {}",
                    error_code
                )));
            }

            Ok(())
        }
    }

    /// Routine control
    pub fn routine_control(
        &self,
        routine_type: u8,
        routine_id: u16,
        params: &[u8],
    ) -> Result<Vec<u8>> {
        unsafe {
            let response = uds_routine_control(
                self.client,
                routine_type,
                routine_id,
                params.as_ptr(),
                params.len() as u32,
            );
            
            if response.is_null() {
                return Err(Sovd2UdsError::UdsCommunication(
                    "Null response received".to_string(),
                ));
            }

            let response_ref = &*response;
            
            if response_ref.error_code != 0 {
                let error = Sovd2UdsError::UdsCommunication(format!(
                    "Routine control failed with error code: {}",
                    response_ref.error_code
                ));
                uds_response_free(response);
                return Err(error);
            }

            let data = if !response_ref.data.is_null() && response_ref.data_length > 0 {
                std::slice::from_raw_parts(response_ref.data, response_ref.data_length as usize)
                    .to_vec()
            } else {
                vec![]
            };

            uds_response_free(response);
            Ok(data)
        }
    }

    /// Get raw client pointer (for advanced use cases)
    pub fn as_ptr(&self) -> *mut uds_client_t {
        self.client
    }
}

impl Drop for UdsClientHandle {
    fn drop(&mut self) {
        unsafe {
            if !self.client.is_null() {
                let _ = self.disconnect();
                uds_client_destroy(self.client);
            }
        }
    }
}

// Ensure thread safety
unsafe impl Send for UdsClientHandle {}
unsafe impl Sync for UdsClientHandle {}

/// Safe wrapper around DoIP client
pub struct DoipClientHandle {
    client: *mut doip_client_t,
}

impl DoipClientHandle {
    /// Create a new DoIP client
    pub fn new(ip_address: &str, port: u16) -> Result<Self> {
        let ip_c = CString::new(ip_address)
            .map_err(|e| Sovd2UdsError::Ffi(format!("Invalid IP address string: {}", e)))?;

        unsafe {
            let client = doip_client_create(ip_c.as_ptr(), port);
            if client.is_null() {
                return Err(Sovd2UdsError::Ffi(
                    "Failed to create DoIP client".to_string(),
                ));
            }
            Ok(Self { client })
        }
    }

    /// Connect to the DoIP target
    pub fn connect(&self) -> Result<()> {
        unsafe {
            let result = doip_client_connect(self.client);
            if result != 0 {
                return Err(Sovd2UdsError::UdsCommunication(format!(
                    "DoIP connection failed with error code: {}",
                    result
                )));
            }
            Ok(())
        }
    }

    /// Disconnect from the DoIP target
    pub fn disconnect(&self) -> Result<()> {
        unsafe {
            let result = doip_client_disconnect(self.client);
            if result != 0 {
                return Err(Sovd2UdsError::UdsCommunication(format!(
                    "DoIP disconnection failed with error code: {}",
                    result
                )));
            }
            Ok(())
        }
    }
}

impl Drop for DoipClientHandle {
    fn drop(&mut self) {
        unsafe {
            if !self.client.is_null() {
                let _ = self.disconnect();
                doip_client_destroy(self.client);
            }
        }
    }
}

unsafe impl Send for DoipClientHandle {}
unsafe impl Sync for DoipClientHandle {}
