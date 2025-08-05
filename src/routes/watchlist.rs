use axum::{
    extract::Json as JsonExtract,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use crate::models::{ApiResponse, Watchlist, AddWatchlistRequest, RemoveWatchlistRequest};
use crate::services::watchlist_service::WatchlistService;

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_watchlist))
        .route("/add", post(add_ticker))
        .route("/remove", post(remove_ticker))
}

async fn get_watchlist() -> Result<Json<ApiResponse<Watchlist>>, StatusCode> {
    let service = WatchlistService::new();
    
    match service.get_watchlist() {
        Ok(watchlist) => Ok(Json(ApiResponse {
            success: true,
            data: Some(watchlist),
            error: None,
        })),
        Err(e) => {
            tracing::error!("Failed to get watchlist: {}", e);
            Ok(Json(ApiResponse {
                success: false,
                data: None,
                error: Some(format!("Failed to get watchlist: {}", e)),
            }))
        }
    }
}

async fn add_ticker(
    JsonExtract(request): JsonExtract<AddWatchlistRequest>,
) -> Result<Json<ApiResponse<Watchlist>>, StatusCode> {
    let service = WatchlistService::new();
    
    match service.add_ticker(request).await {
        Ok(watchlist) => Ok(Json(ApiResponse {
            success: true,
            data: Some(watchlist),
            error: None,
        })),
        Err(e) => {
            tracing::error!("Failed to add ticker: {}", e);
            Ok(Json(ApiResponse {
                success: false,
                data: None,
                error: Some(format!("Failed to add ticker: {}", e)),
            }))
        }
    }
}

async fn remove_ticker(
    JsonExtract(request): JsonExtract<RemoveWatchlistRequest>,
) -> Result<Json<ApiResponse<Watchlist>>, StatusCode> {
    let service = WatchlistService::new();
    
    match service.remove_ticker(request) {
        Ok(watchlist) => Ok(Json(ApiResponse {
            success: true,
            data: Some(watchlist),
            error: None,
        })),
        Err(e) => {
            tracing::error!("Failed to remove ticker: {}", e);
            Ok(Json(ApiResponse {
                success: false,
                data: None,
                error: Some(format!("Failed to remove ticker: {}", e)),
            }))
        }
    }
} 