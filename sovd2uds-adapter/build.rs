use std::env;
use std::path::PathBuf;

fn main() {
    // Get the path to C library headers
    // These paths should be adjusted based on where libudsclient and libdoipclient are located
    let uds_include_path = env::var("UDS_INCLUDE_PATH")
        .unwrap_or_else(|_| "../libudsclient/include".to_string());
    let doip_include_path = env::var("DOIP_INCLUDE_PATH")
        .unwrap_or_else(|_| "../libdoipclient/include".to_string());

    // Tell cargo to tell rustc to link the UDS and DoIP libraries
    println!("cargo:rustc-link-lib=udsclient");
    println!("cargo:rustc-link-lib=doipclient");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // Check if header file exists, if not create a placeholder
    let wrapper_path = PathBuf::from("wrapper.h");
    if !wrapper_path.exists() {
        println!("cargo:warning=wrapper.h not found, FFI bindings will be generated with placeholders");
        std::fs::write(&wrapper_path, r#"
// Placeholder header file for UDS and DoIP client libraries
// Replace this with actual header files from libudsclient and libdoipclient

#ifndef SOVD2UDS_WRAPPER_H
#define SOVD2UDS_WRAPPER_H

#include <stdint.h>
#include <stdbool.h>

// UDS Client structures and functions (placeholder)
typedef struct uds_client uds_client_t;
typedef struct uds_response {
    uint8_t service_id;
    uint8_t *data;
    uint32_t data_length;
    int32_t error_code;
} uds_response_t;

// DoIP Client structures and functions (placeholder)
typedef struct doip_client doip_client_t;

// Function declarations (placeholders - replace with actual library functions)
uds_client_t* uds_client_create(const char* interface, uint32_t address, uint32_t timeout);
void uds_client_destroy(uds_client_t* client);
int32_t uds_client_connect(uds_client_t* client);
int32_t uds_client_disconnect(uds_client_t* client);
uds_response_t* uds_read_data_by_identifier(uds_client_t* client, uint16_t did);
uds_response_t* uds_write_data_by_identifier(uds_client_t* client, uint16_t did, const uint8_t* data, uint32_t length);
uds_response_t* uds_diagnostic_session_control(uds_client_t* client, uint8_t session_type);
uds_response_t* uds_ecu_reset(uds_client_t* client, uint8_t reset_type);
uds_response_t* uds_security_access(uds_client_t* client, uint8_t access_type, const uint8_t* key, uint32_t key_length);
uds_response_t* uds_read_dtc_information(uds_client_t* client, uint8_t sub_function);
uds_response_t* uds_clear_diagnostic_information(uds_client_t* client, uint32_t group);
uds_response_t* uds_routine_control(uds_client_t* client, uint8_t routine_type, uint16_t routine_id, const uint8_t* params, uint32_t params_length);
void uds_response_free(uds_response_t* response);

doip_client_t* doip_client_create(const char* ip_address, uint16_t port);
void doip_client_destroy(doip_client_t* client);
int32_t doip_client_connect(doip_client_t* client);
int32_t doip_client_disconnect(doip_client_t* client);

#endif // SOVD2UDS_WRAPPER_H
"#).expect("Failed to create wrapper.h");
    }

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        .header("wrapper.h")
        // Add include paths for C libraries
        .clang_arg(format!("-I{}", uds_include_path))
        .clang_arg(format!("-I{}", doip_include_path))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
