use axum::{
    extract::Json as JsonExtract,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use crate::models::{ApiResponse, Portfolio, AddPositionRequest, RemovePositionRequest};
use crate::services::portfolio_service::PortfolioService;

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_portfolio))
        .route("/add", post(add_position))
        .route("/remove", post(remove_position))
}

async fn get_portfolio() -> Result<Json<ApiResponse<Portfolio>>, StatusCode> {
    let service = PortfolioService::new();
    
    match service.get_portfolio() {
        Ok(portfolio) => Ok(Json(ApiResponse {
            success: true,
            data: Some(portfolio),
            error: None,
        })),
        Err(e) => {
            tracing::error!("Failed to get portfolio: {}", e);
            Ok(Json(ApiResponse {
                success: false,
                data: None,
                error: Some(format!("Failed to get portfolio: {}", e)),
            }))
        }
    }
}

async fn add_position(
    JsonExtract(request): JsonExtract<AddPositionRequest>,
) -> Result<Json<ApiResponse<Portfolio>>, StatusCode> {
    let service = PortfolioService::new();
    
    match service.add_position(request) {
        Ok(portfolio) => Ok(Json(ApiResponse {
            success: true,
            data: Some(portfolio),
            error: None,
        })),
        Err(e) => {
            tracing::error!("Failed to add position: {}", e);
            Ok(Json(ApiResponse {
                success: false,
                data: None,
                error: Some(format!("Failed to add position: {}", e)),
            }))
        }
    }
}

async fn remove_position(
    JsonExtract(request): JsonExtract<RemovePositionRequest>,
) -> Result<Json<ApiResponse<Portfolio>>, StatusCode> {
    let service = PortfolioService::new();
    
    match service.remove_position(request) {
        Ok(portfolio) => Ok(Json(ApiResponse {
            success: true,
            data: Some(portfolio),
            error: None,
        })),
        Err(e) => {
            tracing::error!("Failed to remove position: {}", e);
            Ok(Json(ApiResponse {
                success: false,
                data: None,
                error: Some(format!("Failed to remove position: {}", e)),
            }))
        }
    }
} 