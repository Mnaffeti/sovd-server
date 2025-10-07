// Example usage of the SOVD2UDS adapter

use sovd2uds_adapter::{
    config::Config,
    translation::SovdUdsTranslator,
    uds::{UdsClient, UdsClientPool},
    models::uds::DiagnosticSessionType,
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = Arc::new(Config::load()?);

    // Create translator
    let translator = SovdUdsTranslator::new();

    // Create UDS client pool
    let pool = UdsClientPool::new(Arc::clone(&config));

    // Example 1: Read VIN
    println!("=== Example 1: Read VIN ===");
    let engine_client = pool.get_client("engine").await?;
    let vin_data = translator.read_data_item(&engine_client, "vin").await?;
    println!("VIN: {:?}", vin_data);

    // Example 2: Read software version
    println!("\n=== Example 2: Read Software Version ===");
    let sw_version = engine_client.get_software_version().await?;
    println!("Software Version: {}", sw_version);

    // Example 3: Change diagnostic session
    println!("\n=== Example 3: Change Diagnostic Session ===");
    engine_client
        .diagnostic_session_control(DiagnosticSessionType::ExtendedDiagnosticSession)
        .await?;
    println!("Diagnostic session changed successfully");

    // Example 4: Read DTCs
    println!("\n=== Example 4: Read DTCs ===");
    let dtc_data = engine_client.read_dtc_information(0x02).await?;
    println!("DTC Data: {} bytes", dtc_data.len());

    // Cleanup
    pool.close_all().await?;

    Ok(())
}
