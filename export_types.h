 */
#ifndef UDS_CLIENT_TYPES_H
#define UDS_CLIENT_TYPES_H

#include <stdint.h>
#include <stddef.h>

/**
 * @brief UDS Diagnostic Session Control (0x10) Service Sub-function Definitions
 *
 * These macros define the hexadecimal values for each sub-function of the
 * UDS Diagnostic Session Control service (Service ID 0x10).
 * They are intended for use in C/C++ embedded development for clarity and
 * maintainability when implementing UDS diagnostic stacks or applications.
 */
#define M_UDS_SESSION_DEFAULT 0x01
#define M_UDS_SESSION_PROGRAMMING 0x02
#define M_UDS_SESSION_EXTENDED 0x03
#define M_UDS_SESSION_SAFETY 0x04



// UDS Service Identifiers (SID)
#define M_UDS_SID_DIAGNOSTIC_SESSION_CONTROL 0x10
#define M_UDS_SID_ECU_RESET 0x11
#define M_UDS_SID_CLEAR_DIAGNOSTIC_INFORMATION 0x14
#define M_UDS_SID_READ_DTC_INFORMATION 0x19
#define M_UDS_SID_READ_DATA_BY_IDENTIFIER 0x22
#define M_UDS_SID_READ_MEMORY_BY_ADDRESS 0x23
#define M_UDS_SID_READ_SCALING_DATA_BY_IDENTIFIER 0x24
#define M_UDS_SID_SECURITY_ACCESS 0x27
#define M_UDS_SID_COMMUNICATION_CONTROL 0x28
#define M_UDS_SID_AUTHENTICATION 0x29
#define M_UDS_SID_RESPONSE_ON_EVENT 0x86
#define M_UDS_SID_READ_DATA_BY_PERIODIC_IDENTIFIER 0x2A
#define M_UDS_SID_DYNAMICALLY_DEFINE_DATA_IDENTIFIER 0x2C
#define M_UDS_SID_WRITE_DATA_BY_IDENTIFIER 0x2E
#define M_UDS_SID_INPUT_OUTPUT_CONTROL_BY_IDENTIFIER 0x2F
#define M_UDS_SID_ROUTINE_CONTROL 0x31
#define M_UDS_SID_REQUEST_DOWNLOAD 0x34
#define M_UDS_SID_REQUEST_UPLOAD 0x35
#define M_UDS_SID_TRANSFER_DATA 0x36
#define M_UDS_SID_REQUEST_TRANSFER_EXIT 0x37
#define M_UDS_SID_REQUEST_FILE_TRANSFER 0x38
#define M_UDS_SID_WRITE_DATA_BY_LOCAL_IDENTIFIER 0x3B
#define M_UDS_SID_WRITE_MEMORY_BY_ADDRESS 0x3D
#define M_UDS_SID_TESTER_PRESENT 0x3E
#define M_UDS_SID_ACCESS_TIMING_PARAMETER 0x83
#define M_UDS_SID_SECURED_DATA_TRANSMISSION 0x84
#define M_UDS_SID_CONTROL_DTC_SETTING 0x85
#define M_UDS_SID_RESPONSE_ON_EVENT 0x86
#define M_UDS_SID_LINK_CONTROL 0x87

/**
 * @brief UDS LinkControl (0x87) Service Sub-function Definitions
 */
#define M_UDS_SUBFUNCTION_VERIFY_MODE_TRANSITION_WITH_FIXED_PARAMETER       0x01U
#define M_UDS_SUBFUNCTION_VERIFY_MODE_TRANSITION_WITH_SPECIFIC_PARAMETER    0x02U
#define M_UDS_SUBFUNCTION_TRANSITION_MODE                                   0x03U

/**
 * @brief UDS LinkControl (0x87) Service LinkControlRecord Definitions
 *
 * These macros define the values for the linkControlRecord parameter used with
 * the verifyModeTransitionWithFixedParameter (0x01) sub-function.
 */
#define M_UDS_LINK_CONTROL_BAUDRATE_9600        0x01U
#define M_UDS_LINK_CONTROL_BAUDRATE_19200       0x02U
#define M_UDS_LINK_CONTROL_BAUDRATE_38400       0x03U
#define M_UDS_LINK_CONTROL_BAUDRATE_57600       0x04U
#define M_UDS_LINK_CONTROL_BAUDRATE_115200      0x05U

/**
 * @brief UDS ReadDTCInformation (0x19) Service Sub-function Definitions
 *
 * These macros define the hexadecimal values for each sub-function of the
 * UDS ReadDTCInformation service (Service ID 0x19).
 * They are intended for use in C/C++ embedded development for clarity and
 * maintainability when implementing UDS diagnostic stacks or applications.
 */


#define M_UDS_SUBFUNCTION_REPORT_NUMBER_OF_DTC_BY_STATUS_MASK           0x01U
#define M_UDS_SUBFUNCTION_REPORT_DTC_BY_STATUS_MASK                     0x02U
#define M_UDS_SUBFUNCTION_REPORT_DTC_SNAPSHOT_RECORD_BY_DTC_NUMBER      0x03U
#define M_UDS_SUBFUNCTION_REPORT_DTC_SNAPSHOT_RECORD_BY_RECORD_NUMBER   0x04U
#define M_UDS_SUBFUNCTION_REPORT_DTC_STORED_DATA_BY_RECORD_NUMBER       0x05U
#define M_UDS_SUBFUNCTION_REPORT_DTC_EXTENDED_DATA_RECORD_BY_DTC_NUMBER 0x06U
#define M_UDS_SUBFUNCTION_REPORT_DTC_BY_SEVERITY_MASK_RECORD            0x07U
#define M_UDS_SUBFUNCTION_REPORT_NUMBER_OF_DTC_BY_SEVERITY_MASK_RECORD  0x08U
#define M_UDS_SUBFUNCTION_REPORT_DTC_SEVERITY_INFORMATION               0x09U
#define M_UDS_SUBFUNCTION_REPORT_SUPPORTED_DTC                          0x0AU
#define M_UDS_SUBFUNCTION_REPORT_DTC_BY_FUNCTIONAL_UNIT                 0x0BU
#define M_UDS_SUBFUNCTION_REPORT_FIRST_TEST_FAILED_DTC                  0x0BU
#define M_UDS_SUBFUNCTION_REPORT_FIRST_CONFIRMED_DTC                    0x0CU
#define M_UDS_SUBFUNCTION_REPORT_MOST_RECENT_TEST_FAILED_DTC            0x0DU
#define M_UDS_SUBFUNCTION_REPORT_MOST_RECENT_CONFIRMED_DTC              0x0EU
#define M_UDS_SUBFUNCTION_REPORT_DTC_FAULT_DETECTION_COUNTER            0x14U
#define M_UDS_SUBFUNCTION_REPORT_DTC_WITH_PERMANENT_STATUS              0x15U
#define M_UDS_SUBFUNCTION_REPORT_DTC_EXT_DATA_RECORD_BY_DTC_NUMBER      0x16U
#define M_UDS_SUBFUNCTION_REPORT_USER_DEF_MEMORY_DTC_BY_STATUS_MASK     0x17U
#define M_UDS_SUBFUNCTION_REPORT_USER_DEF_MEMORY_DTC_SNAPSHOT_RECORD_BY_DTC_NUMBER 0x18U
#define M_UDS_SUBFUNCTION_REPORT_USER_DEF_MEMORY_DTC_EXTENDED_DATA_RECORD_BY_DTC_NUMBER 0x19U
#define M_UDS_SUBFUNCTION_REPORT_SUPPORTED_DTC_EXT_DATA_RECORD          0x1AU
#define M_UDS_SUBFUNCTION_REPORT_WWHOBDDTC_BY_MASK_RECORD               0x42U
#define M_UDS_SUBFUNCTION_REPORT_WWHOBDDTC_WITH_PERMANENT_STATUS        0x55U
#define M_UDS_SUBFUNCTION_REPORT_DTC_INFORMATION_BY_DTC_READINESS_GROUP_IDENTIFIER 0x56U



// UDS Negative Response Codes (NRC) and Strings
#define M_UDS_NRC_CODE_10 0x10
#define M_UDS_NRC_GENERAL_REJECT "General Reject"
#define M_UDS_NRC_CODE_11 0x11
#define M_UDS_NRC_SERVICE_NOT_SUPPORTED "Service Not Supported"
#define M_UDS_NRC_CODE_12 0x12
#define M_UDS_NRC_SUB_FUNCTION_NOT_SUPPORTED "Sub-function Not Supported"
#define M_UDS_NRC_CODE_13 0x13
#define M_UDS_NRC_INCORRECT_MESSAGE_LENGTH_OR_INVALID_FORMAT "Incorrect Message Length or Invalid Format"
#define M_UDS_NRC_CODE_14 0x14
#define M_UDS_NRC_RESPONSE_TOO_LONG "Response Too Long"
#define M_UDS_NRC_CODE_21 0x21
#define M_UDS_NRC_BUSY_REPEAT_REQUEST "Busy Repeat Request"
#define M_UDS_NRC_CODE_22 0x22
#define M_UDS_NRC_CONDITIONS_NOT_CORRECT "Conditions Not Correct"
#define M_UDS_NRC_CODE_24 0x24
#define M_UDS_NRC_REQUEST_SEQUENCE_ERROR "Request Sequence Error"
#define M_UDS_NRC_CODE_25 0x25
#define M_UDS_NRC_NO_RESPONSE_FROM_SUBNET_COMPONENT "No Response From Sub-net Component"
#define M_UDS_NRC_CODE_26 0x26
#define M_UDS_NRC_FAILURE_PREVENTS_EXECUTION "Failure Prevents Execution Of Requested Action"
#define M_UDS_NRC_CODE_31 0x31
#define M_UDS_NRC_REQUEST_OUT_OF_RANGE "Request Out Of Range"
#define M_UDS_NRC_CODE_33 0x33
#define M_UDS_NRC_SECURITY_ACCESS_DENIED "Security Access Denied"
#define M_UDS_NRC_CODE_35 0x35
#define M_UDS_NRC_INVALID_KEY "Invalid Key"
#define M_UDS_NRC_CODE_36 0x36
#define M_UDS_NRC_EXCEED_NUMBER_OF_ATTEMPTS "Exceed Number Of Attempts"
#define M_UDS_NRC_CODE_37 0x37
#define M_UDS_NRC_REQUIRED_TIME_DELAY_NOT_EXPIRED "Required Time Delay Not Expired"
#define M_UDS_NRC_CODE_70 0x70
#define M_UDS_NRC_UPLOAD_DOWNLOAD_NOT_ACCEPTED "Upload Download Not Accepted"
#define M_UDS_NRC_CODE_71 0x71
#define M_UDS_NRC_TRANSFER_DATA_SUSPENDED "Transfer Data Suspended"
#define M_UDS_NRC_CODE_72 0x72
#define M_UDS_NRC_GENERAL_PROGRAMMING_FAILURE "General Programming Failure"
#define M_UDS_NRC_CODE_73 0x73
#define M_UDS_NRC_WRONG_BLOCK_SEQUENCE_COUNTER "Wrong Block Sequence Counter"
#define M_UDS_NRC_CODE_78 0x78
#define M_UDS_NRC_RESPONSE_PENDING "Request Correctly Received-Response Pending"
#define M_UDS_NRC_CODE_7E 0x7E
#define M_UDS_NRC_SUB_FUNCTION_NOT_SUPPORTED_IN_ACTIVE_SESSION "Sub-function Not Supported In Active Session"
#define M_UDS_NRC_CODE_7F 0x7F
#define M_UDS_NRC_SERVICE_NOT_SUPPORTED_IN_ACTIVE_SESSION "Service Not Supported In Active Session"
#define M_UDS_NRC_CODE_81 0x81
#define M_UDS_NRC_RPM_TOO_HIGH "RPM Too High"
#define M_UDS_NRC_CODE_82 0x82
#define M_UDS_NRC_RPM_TOO_LOW "RPM Too Low"
#define M_UDS_NRC_CODE_83 0x83
#define M_UDS_NRC_ENGINE_IS_RUNNING "Engine is Running"
#define M_UDS_NRC_CODE_84 0x84
#define M_UDS_NRC_ENGINE_IS_NOT_RUNNING "Engine is Not Running"
#define M_UDS_NRC_CODE_85 0x85
#define M_UDS_NRC_ENGINE_RUN_TIME_TOO_LOW "Engine Run Time Too Low"
#define M_UDS_NRC_CODE_86 0x86
#define M_UDS_NRC_TEMPERATURE_TOO_HIGH "Temperature Too High"
#define M_UDS_NRC_CODE_87 0x87
#define M_UDS_NRC_TEMPERATURE_TOO_LOW "Temperature Too Low"
#define M_UDS_NRC_CODE_88 0x88
#define M_UDS_NRC_VEHICLE_SPEED_TOO_HIGH "Vehicle Speed Too High"
#define M_UDS_NRC_CODE_89 0x89
#define M_UDS_NRC_VEHICLE_SPEED_TOO_LOW "Vehicle Speed Too Low"
#define M_UDS_NRC_CODE_8A 0x8A
#define M_UDS_NRC_THROTTLE_PEDAL_TOO_HIGH "Throttle/Pedal Too High"
#define M_UDS_NRC_CODE_8B 0x8B
#define M_UDS_NRC_THROTTLE_PEDAL_TOO_LOW "Throttle/Pedal Too Low"
#define M_UDS_NRC_CODE_8C 0x8C
#define M_UDS_NRC_TRANSMISSION_RANGE_NOT_IN_NEUTRAL "Transmission Range Not in Neutral"
#define M_UDS_NRC_CODE_8D 0x8D
#define M_UDS_NRC_TRANSMISSION_RANGE_NOT_IN_GEAR "Transmission Range Not in Gear"
#define M_UDS_NRC_CODE_8F 0x8F
#define M_UDS_NRC_BRAKE_SWITCHES_NOT_CLOSED "Brake Switch(es) Not Closed"
#define M_UDS_NRC_CODE_90 0x90
#define M_UDS_NRC_SHIFTER_LEVER_NOT_IN_PARK "Shifter Lever Not in Park"
#define M_UDS_NRC_CODE_91 0x91
#define M_UDS_NRC_TORQUE_CONVERTER_CLUTCH_LOCKED "Torque Converter Clutch Locked"
#define M_UDS_NRC_CODE_92 0x92
#define M_UDS_NRC_VOLTAGE_TOO_HIGH "Voltage Too High"
#define M_UDS_NRC_CODE_93 0x93
#define M_UDS_NRC_VOLTAGE_TOO_LOW "Voltage Too Low"
#define M_UDS_NRC_UNKNOWN "Unknown NRC"


/**
 * @brief Callback function for periodic data reception.
 * @ingroup uds_client_api_periodic_read
 * @param did The Data Identifier (DID) for which data was received.
 * @param data Pointer to the buffer containing the received data.
 * @param data_len Length of the received data.
 */
typedef void (*uds_periodic_cb_t)(uint8_t did, const uint8_t* data, size_t data_len);


/**
 * @brief Defines the event types for the ResponseOnEvent (0x86) service.
 */
#define UDS_EVENT_TYPE_STOP_REPORTING               0x00
#define UDS_EVENT_TYPE_ON_DTC_STATUS_CHANGE         0x01
#define UDS_EVENT_TYPE_ON_TIMER_INTERRUPT           0x02
#define UDS_EVENT_TYPE_ON_CHANGE_OF_DATA_IDENTIFIER 0x03
#define UDS_EVENT_TYPE_REPORT_ACTIVATED_EVENTS      0x04
#define UDS_EVENT_TYPE_START_REPORTING              0x05
#define UDS_EVENT_TYPE_CLEAR_REPORTING              0x06
#define UDS_EVENT_TYPE_ON_COMPARISON_OF_VALUES      0x07

#define UDS_ROE_SUPPRESS_RSP_MSG_INDICATION_MSK     0x80 //bit 7
#define UDS_ROE_STORE_EVENT_MSK                     0x40 //bit 6
#define UDS_ROE_EVENT_TYPE_MSK                      0x3F
 
/**
 * @brief Callback function type for receiving asynchronous event data from the ResponseOnEvent service.
 * @param event_type The type of event that triggered the response.
 * @param data Pointer to the response data payload.
 * @param len Length of the response data payload.
 */
typedef void (*uds_event_cb_t)(uint8_t event_type, const uint8_t* data, size_t len);


/**
 * @brief Defines the transmission rates for periodic data reading in milliseconds.
 */
#define UDS_PERIODIC_RATE_SLOW_MS 1000
#define UDS_PERIODIC_RATE_MEDIUM_MS 300
#define UDS_PERIODIC_RATE_FAST_MS 100

/**
 * @brief Defines the transmission modes for periodic data reading.
 */
#define UDS_TRANSMISSION_MODE_STOP 0x04
#define UDS_TRANSMISSION_MODE_SLOW 0x01
#define UDS_TRANSMISSION_MODE_MEDIUM 0x02
#define UDS_TRANSMISSION_MODE_FAST 0x03

/**
 * @brief Structure to hold the details of an activated event reported by the ECU.
 */
#define MAX_EVENT_TYPE_RECORD_SIZE 16
#define MAX_SERVICE_RECORD_SIZE 32

typedef struct {
    uint8_t eventType;
    uint8_t eventWindowTime;
    uint8_t eventTypeRecord[MAX_EVENT_TYPE_RECORD_SIZE];
    size_t eventTypeRecordSize;
    uint8_t serviceToRespondToRecord[MAX_SERVICE_RECORD_SIZE];
    size_t serviceToRespondToRecordSize;
} T_stUdsActivatedEvent;

#endif // UDS_CLIENT_TYPES_H
