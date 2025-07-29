use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;


use crate::models::*;
use crate::services::FinanceService;

#[derive(Debug, Deserialize)]
pub struct HistoryQuery {
    pub days: Option<u32>,
}

pub async fn health_check() -> Json<ApiResponse<String>> {
    Json(ApiResponse::success("Finance API is running!".to_string()))
}

pub async fn get_stock_price(Path(symbol): Path<String>) -> Result<Json<ApiResponse<StockPrice>>, (StatusCode, Json<ErrorResponse>)> {
    match FinanceService::get_stock_price(&symbol) {
        Ok(stock_price) => Ok(Json(ApiResponse::success(stock_price))),
        Err(e) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new(format!("Failed to get stock price for {}: {}", symbol, e))),
        )),
    }
}

pub async fn get_stock_history(
    Path(symbol): Path<String>,
    Query(query): Query<HistoryQuery>,
) -> Result<Json<ApiResponse<StockHistory>>, (StatusCode, Json<ErrorResponse>)> {
    let days = query.days.unwrap_or(30);
    
    if days > 365 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new("Days parameter cannot exceed 365".to_string())),
        ));
    }

    match FinanceService::get_stock_history(&symbol, days) {
        Ok(history) => Ok(Json(ApiResponse::success(history))),
        Err(e) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new(format!("Failed to get stock history for {}: {}", symbol, e))),
        )),
    }
}

pub async fn get_portfolio() -> Result<Json<ApiResponse<Vec<PortfolioItem>>>, (StatusCode, Json<ErrorResponse>)> {
    match FinanceService::get_portfolio() {
        Ok(portfolio) => Ok(Json(ApiResponse::success(portfolio))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::new(format!("Failed to get portfolio: {}", e))),
        )),
    }
}

pub async fn add_to_portfolio(
    Json(request): Json<AddToPortfolioRequest>,
) -> Result<Json<ApiResponse<PortfolioItem>>, (StatusCode, Json<ErrorResponse>)> {
    if request.shares <= 0.0 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new("Shares must be greater than 0".to_string())),
        ));
    }

    if request.price_per_share <= 0.0 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new("Price per share must be greater than 0".to_string())),
        ));
    }

    match FinanceService::add_to_portfolio(request) {
        Ok(item) => Ok(Json(ApiResponse::success(item))),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(format!("Failed to add to portfolio: {}", e))),
        )),
    }
}

pub async fn get_watchlist() -> Result<Json<ApiResponse<Vec<WatchlistItem>>>, (StatusCode, Json<ErrorResponse>)> {
    match FinanceService::get_watchlist() {
        Ok(watchlist) => Ok(Json(ApiResponse::success(watchlist))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::new(format!("Failed to get watchlist: {}", e))),
        )),
    }
}

pub async fn add_to_watchlist(
    Json(request): Json<AddToWatchlistRequest>,
) -> Result<Json<ApiResponse<WatchlistItem>>, (StatusCode, Json<ErrorResponse>)> {
    if request.symbol.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new("Symbol cannot be empty".to_string())),
        ));
    }

    match FinanceService::add_to_watchlist(request) {
        Ok(item) => Ok(Json(ApiResponse::success(item))),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::new(format!("Failed to add to watchlist: {}", e))),
        )),
    }
}

pub async fn get_market_summary() -> Result<Json<ApiResponse<MarketSummary>>, (StatusCode, Json<ErrorResponse>)> {
    match FinanceService::get_market_summary() {
        Ok(summary) => Ok(Json(ApiResponse::success(summary))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::new(format!("Failed to get market summary: {}", e))),
        )),
    }
} 