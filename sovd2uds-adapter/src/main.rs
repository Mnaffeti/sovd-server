mod config;
mod error;
mod ffi;
mod models;
mod server;
mod translation;
mod uds;

use config::Config;
use server::{create_router, AppState};
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing::{info, Level};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use translation::SovdUdsTranslator;
use uds::UdsClientPool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = Config::load().unwrap_or_else(|e| {
        eprintln!("Failed to load config, using defaults: {}", e);
        Config::default()
    });

    // Initialize logging
    init_logging(&config);

    info!("Starting SOVD2UDS Adapter v{}", env!("CARGO_PKG_VERSION"));
    info!("Configuration loaded successfully");

    // Create shared state
    let config = Arc::new(config);
    let translator = Arc::new(SovdUdsTranslator::new());
    let client_pool = Arc::new(UdsClientPool::new(Arc::clone(&config)));

    let state = AppState {
        config: Arc::clone(&config),
        translator,
        client_pool: Arc::clone(&client_pool),
    };

    // Build the router
    let app = create_router(state).layer(TraceLayer::new_for_http());

    // Server address
    let addr = format!("{}:{}", config.server.host, config.server.port);
    info!("Server listening on http://{}", addr);
    info!("API available at http://{}/api/v1", addr);

    // Start the server
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    
    info!("SOVD2UDS Adapter is ready to accept connections");
    
    axum::serve(listener, app)
        .await
        .map_err(|e| {
            tracing::error!("Server error: {}", e);
            e
        })?;

    // Cleanup on shutdown
    info!("Shutting down SOVD2UDS Adapter");
    client_pool.close_all().await?;

    Ok(())
}

/// Initialize logging based on configuration
fn init_logging(config: &Config) {
    let log_level = match config.logging.level.to_lowercase().as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(log_level.to_string()));

    match config.logging.format.as_str() {
        "json" => {
            tracing_subscriber::registry()
                .with(filter)
                .with(fmt::layer().json())
                .init();
        }
        _ => {
            tracing_subscriber::registry()
                .with(filter)
                .with(fmt::layer().pretty())
                .init();
        }
    }
}
