use crate::config::Config;
use crate::error::Sovd2UdsError;
use crate::models::*;
use crate::translation::SovdUdsTranslator;
use crate::uds::UdsClientPool;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;
use tracing::{error, info};

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub translator: Arc<SovdUdsTranslator>,
    pub client_pool: Arc<UdsClientPool>,
}

/// Query parameters for component data endpoint
#[derive(Debug, Deserialize)]
pub struct DataQuery {
    categories: Option<String>,
}

/// Create the API router
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/api/v1/components", get(get_components))
        .route(
            "/api/v1/components/:component_id/data",
            get(get_component_data),
        )
        .route(
            "/api/v1/components/:component_id/data/:data_id",
            get(get_data_item_value),
        )
        .route(
            "/api/v1/components/:component_id/actuators/control",
            post(control_actuator),
        )
        .route(
            "/api/v1/components/:component_id/dtcs",
            post(manage_dtcs),
        )
        .route(
            "/api/v1/components/:component_id/services",
            post(execute_service),
        )
        .route("/health", get(health_check))
        .with_state(state)
}

/// Health check endpoint
async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "sovd2uds-adapter"
    }))
}

/// Get all components
async fn get_components(
    State(state): State<AppState>,
) -> Result<Json<ComponentsResponse>, AppError> {
    info!("Getting all components");

    let components = state.translator.get_components();

    Ok(Json(ComponentsResponse { components }))
}

/// Get component data items
async fn get_component_data(
    State(state): State<AppState>,
    Path(component_id): Path<String>,
    Query(query): Query<DataQuery>,
) -> Result<Json<DataItemsResponse>, AppError> {
    info!("Getting data items for component: {}", component_id);

    // Parse categories
    let categories = query
        .categories
        .as_ref()
        .map(|s| s.split(',').map(|c| c.trim().to_string()).collect::<Vec<_>>());

    let items = state.translator.get_component_data_items(categories.as_deref());

    Ok(Json(DataItemsResponse { items }))
}

/// Get specific data item value
async fn get_data_item_value(
    State(state): State<AppState>,
    Path((component_id, data_id)): Path<(String, String)>,
) -> Result<Json<DataItemValue>, AppError> {
    info!(
        "Getting data item '{}' for component '{}'",
        data_id, component_id
    );

    // Get UDS client
    let client = state.client_pool.get_client(&component_id).await?;

    // Read data item via translator
    let value = state.translator.read_data_item(&client, &data_id).await?;

    Ok(Json(value))
}

/// Control actuator
async fn control_actuator(
    State(state): State<AppState>,
    Path(component_id): Path<String>,
    Json(request): Json<ActuatorControlRequest>,
) -> Result<Json<ActuatorControlResponse>, AppError> {
    info!(
        "Controlling actuator '{}' on component '{}': action={}",
        request.actuator_id, component_id, request.action
    );

    // Get UDS client
    let client = state.client_pool.get_client(&component_id).await?;

    // Execute actuator control
    let response = state.translator.control_actuator(&client, &request).await?;

    Ok(Json(response))
}

/// Manage DTCs
async fn manage_dtcs(
    State(state): State<AppState>,
    Path(component_id): Path<String>,
    Json(request): Json<DtcManagementRequest>,
) -> Result<Json<DtcManagementResponse>, AppError> {
    info!(
        "Managing DTCs for component '{}': action={}",
        component_id, request.action
    );

    // Get UDS client
    let client = state.client_pool.get_client(&component_id).await?;

    // Manage DTCs
    let response = state.translator.manage_dtcs(&client, &request).await?;

    Ok(Json(response))
}

/// Execute generic service
async fn execute_service(
    State(state): State<AppState>,
    Path(component_id): Path<String>,
    Json(request): Json<ServiceRequest>,
) -> Result<Json<ServiceResponse>, AppError> {
    info!(
        "Executing service '{}' on component '{}'",
        request.service_type, component_id
    );

    // Get UDS client
    let client = state.client_pool.get_client(&component_id).await?;

    // Execute based on service type
    let response = match request.service_type.as_str() {
        "session_control" => {
            // Extract session type from parameters
            let session_type = request
                .parameters
                .as_ref()
                .and_then(|p| p.get("session_type"))
                .and_then(|v| v.as_u64())
                .ok_or_else(|| {
                    Sovd2UdsError::InvalidRequest("Missing session_type parameter".to_string())
                })? as u8;

            let session_type_enum = match session_type {
                0x01 => DiagnosticSessionType::DefaultSession,
                0x02 => DiagnosticSessionType::ProgrammingSession,
                0x03 => DiagnosticSessionType::ExtendedDiagnosticSession,
                0x04 => DiagnosticSessionType::SafetySystemDiagnosticSession,
                _ => {
                    return Err(AppError::from(Sovd2UdsError::InvalidRequest(
                        format!("Invalid session type: 0x{:02X}", session_type),
                    )))
                }
            };

            client.diagnostic_session_control(session_type_enum).await?;

            ServiceResponse {
                service_type: "session_control".to_string(),
                status: "success".to_string(),
                results: Some(serde_json::json!({
                    "session_type": format!("0x{:02X}", session_type)
                })),
                message: Some("Session changed successfully".to_string()),
                timestamp: Some(chrono::Utc::now()),
            }
        }
        "ecu_reset" => {
            let reset_type = request
                .parameters
                .as_ref()
                .and_then(|p| p.get("reset_type"))
                .and_then(|v| v.as_u64())
                .ok_or_else(|| {
                    Sovd2UdsError::InvalidRequest("Missing reset_type parameter".to_string())
                })? as u8;

            let reset_type_enum = match reset_type {
                0x01 => EcuResetType::HardReset,
                0x02 => EcuResetType::KeyOffOnReset,
                0x03 => EcuResetType::SoftReset,
                _ => {
                    return Err(AppError::from(Sovd2UdsError::InvalidRequest(
                        format!("Invalid reset type: 0x{:02X}", reset_type),
                    )))
                }
            };

            client.ecu_reset(reset_type_enum).await?;

            ServiceResponse {
                service_type: "ecu_reset".to_string(),
                status: "success".to_string(),
                results: None,
                message: Some("ECU reset executed".to_string()),
                timestamp: Some(chrono::Utc::now()),
            }
        }
        _ => {
            return Err(AppError::from(Sovd2UdsError::InvalidRequest(format!(
                "Unknown service type: {}",
                request.service_type
            ))))
        }
    };

    Ok(Json(response))
}

/// Error wrapper for axum responses
pub struct AppError(Sovd2UdsError);

impl From<Sovd2UdsError> for AppError {
    fn from(err: Sovd2UdsError) -> Self {
        AppError(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self.0 {
            Sovd2UdsError::ComponentNotFound(_) => (StatusCode::NOT_FOUND, self.0.to_string()),
            Sovd2UdsError::DataItemNotFound(_) => (StatusCode::NOT_FOUND, self.0.to_string()),
            Sovd2UdsError::InvalidRequest(_) => (StatusCode::BAD_REQUEST, self.0.to_string()),
            Sovd2UdsError::Timeout(_) => (StatusCode::REQUEST_TIMEOUT, self.0.to_string()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()),
        };

        error!("Request error: {}", error_message);

        let body = Json(ErrorResponse {
            error: error_message.clone(),
            code: status.as_u16(),
            details: Some(format!("{:?}", self.0)),
        });

        (status, body).into_response()
    }
}
