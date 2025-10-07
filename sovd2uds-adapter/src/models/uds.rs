/// UDS Service Identifiers (SID)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UdsServiceId {
    DiagnosticSessionControl = 0x10,
    EcuReset = 0x11,
    SecurityAccess = 0x27,
    CommunicationControl = 0x28,
    TesterPresent = 0x3E,
    AccessTimingParameter = 0x83,
    SecuredDataTransmission = 0x84,
    ControlDTCSetting = 0x85,
    ResponseOnEvent = 0x86,
    LinkControl = 0x87,
    ReadDataByIdentifier = 0x22,
    ReadMemoryByAddress = 0x23,
    ReadScalingDataByIdentifier = 0x24,
    ReadDataByPeriodicIdentifier = 0x2A,
    DynamicallyDefineDataIdentifier = 0x2C,
    WriteDataByIdentifier = 0x2E,
    WriteMemoryByAddress = 0x3D,
    ClearDiagnosticInformation = 0x14,
    ReadDTCInformation = 0x19,
    InputOutputControlByIdentifier = 0x2F,
    RoutineControl = 0x31,
    RequestDownload = 0x34,
    RequestUpload = 0x35,
    TransferData = 0x36,
    RequestTransferExit = 0x37,
}

impl UdsServiceId {
    pub fn positive_response(&self) -> u8 {
        (*self as u8) + 0x40
    }
}

/// UDS Diagnostic Session Types
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSessionType {
    DefaultSession = 0x01,
    ProgrammingSession = 0x02,
    ExtendedDiagnosticSession = 0x03,
    SafetySystemDiagnosticSession = 0x04,
}

/// UDS ECU Reset Types
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EcuResetType {
    HardReset = 0x01,
    KeyOffOnReset = 0x02,
    SoftReset = 0x03,
    EnableRapidPowerShutDown = 0x04,
    DisableRapidPowerShutDown = 0x05,
}

/// UDS Routine Control Types
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoutineControlType {
    StartRoutine = 0x01,
    StopRoutine = 0x02,
    RequestRoutineResults = 0x03,
}

/// UDS DTC Status Mask
#[derive(Debug, Clone, Copy)]
pub struct DtcStatusMask(pub u8);

impl DtcStatusMask {
    pub const TEST_FAILED: u8 = 0x01;
    pub const TEST_FAILED_THIS_OPERATION_CYCLE: u8 = 0x02;
    pub const PENDING_DTC: u8 = 0x04;
    pub const CONFIRMED_DTC: u8 = 0x08;
    pub const TEST_NOT_COMPLETED_SINCE_LAST_CLEAR: u8 = 0x10;
    pub const TEST_FAILED_SINCE_LAST_CLEAR: u8 = 0x20;
    pub const TEST_NOT_COMPLETED_THIS_OPERATION_CYCLE: u8 = 0x40;
    pub const WARNING_INDICATOR_REQUESTED: u8 = 0x80;
}

/// UDS Request structure
#[derive(Debug, Clone)]
pub struct UdsRequest {
    pub service_id: u8,
    pub data: Vec<u8>,
}

impl UdsRequest {
    pub fn new(service_id: UdsServiceId, data: Vec<u8>) -> Self {
        Self {
            service_id: service_id as u8,
            data,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![self.service_id];
        bytes.extend_from_slice(&self.data);
        bytes
    }
}

/// UDS Response structure
#[derive(Debug, Clone)]
pub struct UdsResponse {
    pub service_id: u8,
    pub data: Vec<u8>,
    pub is_positive: bool,
    pub nrc: Option<u8>, // Negative Response Code
}

impl UdsResponse {
    pub fn new_positive(service_id: u8, data: Vec<u8>) -> Self {
        Self {
            service_id,
            data,
            is_positive: true,
            nrc: None,
        }
    }

    pub fn new_negative(service_id: u8, nrc: u8) -> Self {
        Self {
            service_id,
            data: vec![],
            is_positive: false,
            nrc: Some(nrc),
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.is_empty() {
            return None;
        }

        let response_id = bytes[0];
        
        // Check if it's a negative response (0x7F)
        if response_id == 0x7F {
            if bytes.len() < 3 {
                return None;
            }
            return Some(Self::new_negative(bytes[1], bytes[2]));
        }

        // Positive response
        let data = if bytes.len() > 1 {
            bytes[1..].to_vec()
        } else {
            vec![]
        };

        Some(Self::new_positive(response_id, data))
    }
}

/// Common UDS Data Identifiers (DIDs)
pub mod data_identifiers {
    pub const VIN: u16 = 0xF190;
    pub const ECU_SERIAL_NUMBER: u16 = 0xF18C;
    pub const ECU_MANUFACTURING_DATE: u16 = 0xF18B;
    pub const ECU_HARDWARE_VERSION: u16 = 0xF191;
    pub const ECU_SOFTWARE_VERSION: u16 = 0xF194;
    pub const SYSTEM_SUPPLIER_ID: u16 = 0xF18A;
    pub const ECU_HARDWARE_NUMBER: u16 = 0xF191;
    pub const ECU_SOFTWARE_NUMBER: u16 = 0xF194;
    pub const VEHICLE_MANUFACTURER_ECU_SOFTWARE_NUMBER: u16 = 0xF195;
}
