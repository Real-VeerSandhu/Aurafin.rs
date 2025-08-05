use axum::{
    extract::Path,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use crate::models::{ApiResponse, StockQuote};
use crate::services::research_service::ResearchService;

pub fn router() -> Router {
    Router::new().route("/quote/:ticker", get(get_stock_quote))
}

async fn get_stock_quote(
    Path(ticker): Path<String>,
) -> Result<Json<ApiResponse<StockQuote>>, StatusCode> {
    let service = ResearchService::new();
    
    match service.get_stock_quote(&ticker).await {
        Ok(quote) => Ok(Json(ApiResponse {
            success: true,
            data: Some(quote),
            error: None,
        })),
        Err(e) => {
            tracing::error!("Failed to get stock quote for {}: {}", ticker, e);
            Ok(Json(ApiResponse {
                success: false,
                data: None,
                error: Some(format!("Failed to get stock quote: {}", e)),
            }))
        }
    }
} 