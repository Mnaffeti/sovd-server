use thiserror::Error;

/// Main error type for the SOVD2UDS adapter
#[derive(Error, Debug)]
pub enum Sovd2UdsError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("UDS communication error: {0}")]
    UdsCommunication(String),

    #[error("UDS protocol error: service={service:02X}, nrc={nrc:02X}, description={description}")]
    UdsProtocol {
        service: u8,
        nrc: u8, // Negative Response Code
        description: String,
    },

    #[error("Translation error: {0}")]
    Translation(String),

    #[error("Component not found: {0}")]
    ComponentNotFound(String),

    #[error("Data item not found: {0}")]
    DataItemNotFound(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("FFI error: {0}")]
    Ffi(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

/// UDS Negative Response Codes (NRC)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UdsNegativeResponseCode {
    GeneralReject = 0x10,
    ServiceNotSupported = 0x11,
    SubFunctionNotSupported = 0x12,
    IncorrectMessageLengthOrInvalidFormat = 0x13,
    ResponseTooLong = 0x14,
    BusyRepeatRequest = 0x21,
    ConditionsNotCorrect = 0x22,
    RequestSequenceError = 0x24,
    NoResponseFromSubnetComponent = 0x25,
    FailurePreventsExecutionOfRequestedAction = 0x26,
    RequestOutOfRange = 0x31,
    SecurityAccessDenied = 0x33,
    InvalidKey = 0x35,
    ExceedNumberOfAttempts = 0x36,
    RequiredTimeDelayNotExpired = 0x37,
    UploadDownloadNotAccepted = 0x70,
    TransferDataSuspended = 0x71,
    GeneralProgrammingFailure = 0x72,
    WrongBlockSequenceCounter = 0x73,
    RequestCorrectlyReceivedResponsePending = 0x78,
    SubFunctionNotSupportedInActiveSession = 0x7E,
    ServiceNotSupportedInActiveSession = 0x7F,
}

impl UdsNegativeResponseCode {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x10 => Some(Self::GeneralReject),
            0x11 => Some(Self::ServiceNotSupported),
            0x12 => Some(Self::SubFunctionNotSupported),
            0x13 => Some(Self::IncorrectMessageLengthOrInvalidFormat),
            0x14 => Some(Self::ResponseTooLong),
            0x21 => Some(Self::BusyRepeatRequest),
            0x22 => Some(Self::ConditionsNotCorrect),
            0x24 => Some(Self::RequestSequenceError),
            0x25 => Some(Self::NoResponseFromSubnetComponent),
            0x26 => Some(Self::FailurePreventsExecutionOfRequestedAction),
            0x31 => Some(Self::RequestOutOfRange),
            0x33 => Some(Self::SecurityAccessDenied),
            0x35 => Some(Self::InvalidKey),
            0x36 => Some(Self::ExceedNumberOfAttempts),
            0x37 => Some(Self::RequiredTimeDelayNotExpired),
            0x70 => Some(Self::UploadDownloadNotAccepted),
            0x71 => Some(Self::TransferDataSuspended),
            0x72 => Some(Self::GeneralProgrammingFailure),
            0x73 => Some(Self::WrongBlockSequenceCounter),
            0x78 => Some(Self::RequestCorrectlyReceivedResponsePending),
            0x7E => Some(Self::SubFunctionNotSupportedInActiveSession),
            0x7F => Some(Self::ServiceNotSupportedInActiveSession),
            _ => None,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::GeneralReject => "General reject",
            Self::ServiceNotSupported => "Service not supported",
            Self::SubFunctionNotSupported => "Sub-function not supported",
            Self::IncorrectMessageLengthOrInvalidFormat => "Incorrect message length or invalid format",
            Self::ResponseTooLong => "Response too long",
            Self::BusyRepeatRequest => "Busy, repeat request",
            Self::ConditionsNotCorrect => "Conditions not correct",
            Self::RequestSequenceError => "Request sequence error",
            Self::NoResponseFromSubnetComponent => "No response from subnet component",
            Self::FailurePreventsExecutionOfRequestedAction => "Failure prevents execution of requested action",
            Self::RequestOutOfRange => "Request out of range",
            Self::SecurityAccessDenied => "Security access denied",
            Self::InvalidKey => "Invalid key",
            Self::ExceedNumberOfAttempts => "Exceed number of attempts",
            Self::RequiredTimeDelayNotExpired => "Required time delay not expired",
            Self::UploadDownloadNotAccepted => "Upload/download not accepted",
            Self::TransferDataSuspended => "Transfer data suspended",
            Self::GeneralProgrammingFailure => "General programming failure",
            Self::WrongBlockSequenceCounter => "Wrong block sequence counter",
            Self::RequestCorrectlyReceivedResponsePending => "Request correctly received, response pending",
            Self::SubFunctionNotSupportedInActiveSession => "Sub-function not supported in active session",
            Self::ServiceNotSupportedInActiveSession => "Service not supported in active session",
        }
    }
}

/// Result type for SOVD2UDS operations
pub type Result<T> = std::result::Result<T, Sovd2UdsError>;
