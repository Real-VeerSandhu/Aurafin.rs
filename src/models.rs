use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StockPrice {
    pub symbol: String,
    pub price: f64,
    pub change: f64,
    pub change_percent: f64,
    pub volume: u64,
    pub market_cap: Option<u64>,
    pub high_24h: Option<f64>,
    pub low_24h: Option<f64>,
    pub open: Option<f64>,
    pub previous_close: Option<f64>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StockHistory {
    pub symbol: String,
    pub data: Vec<HistoricalDataPoint>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HistoricalDataPoint {
    pub date: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PortfolioItem {
    pub id: Uuid,
    pub symbol: String,
    pub shares: f64,
    pub average_price: f64,
    pub current_price: f64,
    pub total_value: f64,
    pub gain_loss: f64,
    pub gain_loss_percent: f64,
    pub added_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WatchlistItem {
    pub id: Uuid,
    pub symbol: String,
    pub current_price: f64,
    pub change: f64,
    pub change_percent: f64,
    pub added_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarketSummary {
    pub total_market_cap: u64,
    pub total_volume: u64,
    pub gainers: Vec<StockPrice>,
    pub losers: Vec<StockPrice>,
    pub most_active: Vec<StockPrice>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddToPortfolioRequest {
    pub symbol: String,
    pub shares: f64,
    pub price_per_share: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddToWatchlistRequest {
    pub symbol: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub timestamp: DateTime<Utc>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
            timestamp: Utc::now(),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            message: Some(message),
            timestamp: Utc::now(),
        }
    }
}

impl ErrorResponse {
    pub fn new(error: String) -> Self {
        Self {
            success: false,
            error,
            timestamp: Utc::now(),
        }
    }
} 