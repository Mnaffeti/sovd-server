use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for the SOVD2UDS adapter
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub uds: UdsConfig,
    pub doip: DoipConfig,
    pub logging: LoggingConfig,
    pub components: HashMap<String, u32>,
    pub security: SecurityConfig,
    pub performance: PerformanceConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub request_timeout: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UdsConfig {
    pub interface: String,
    pub default_address: u32,
    pub timeout: u32,
    pub max_retries: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DoipConfig {
    pub enabled: bool,
    pub target_address: String,
    pub port: u16,
    pub source_address: u32,
    pub target_logical_address: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
    pub log_file: String,
    pub log_requests: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SecurityConfig {
    pub require_security_access: bool,
    pub security_level: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PerformanceConfig {
    pub max_concurrent_requests: usize,
    pub connection_pool_size: usize,
}

impl Config {
    /// Load configuration from file and environment variables
    pub fn load() -> Result<Self, config::ConfigError> {
        let builder = config::Config::builder()
            // Start with default config file
            .add_source(config::File::with_name("config.toml").required(false))
            // Override with environment variables (prefix: SOVD2UDS)
            .add_source(
                config::Environment::with_prefix("SOVD2UDS")
                    .separator("__")
                    .try_parsing(true),
            );

        let config = builder.build()?;
        config.try_deserialize()
    }

    /// Get ECU address for a component
    pub fn get_component_address(&self, component_id: &str) -> Option<u32> {
        self.components
            .get(component_id)
            .copied()
            .or(Some(self.uds.default_address))
    }
}

impl Default for Config {
    fn default() -> Self {
        let mut components = HashMap::new();
        components.insert("engine".to_string(), 0x7E0);
        components.insert("transmission".to_string(), 0x7E1);
        components.insert("abs".to_string(), 0x7E2);
        components.insert("airbag".to_string(), 0x7E3);

        Self {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8081,
                request_timeout: 30,
            },
            uds: UdsConfig {
                interface: "can0".to_string(),
                default_address: 0x7E0,
                timeout: 5000,
                max_retries: 3,
            },
            doip: DoipConfig {
                enabled: true,
                target_address: "192.168.1.100".to_string(),
                port: 13400,
                source_address: 0x0E80,
                target_logical_address: 0x1000,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                format: "pretty".to_string(),
                log_file: "sovd2uds-adapter.log".to_string(),
                log_requests: true,
            },
            components,
            security: SecurityConfig {
                require_security_access: false,
                security_level: 0x01,
            },
            performance: PerformanceConfig {
                max_concurrent_requests: 10,
                connection_pool_size: 5,
            },
        }
    }
}
