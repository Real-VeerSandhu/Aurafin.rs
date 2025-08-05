use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StockQuote {
    pub ticker: String,
    pub price: f64,
    pub change: f64,
    pub percent_change: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Position {
    pub ticker: String,
    pub shares: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Portfolio {
    pub positions: Vec<Position>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Watchlist {
    pub tickers: Vec<String>,
}

// Request/Response models
#[derive(Debug, Deserialize)]
pub struct AddPositionRequest {
    pub ticker: String,
    pub shares: u32,
}

#[derive(Debug, Deserialize)]
pub struct RemovePositionRequest {
    pub ticker: String,
    pub shares: u32,
}

#[derive(Debug, Deserialize)]
pub struct AddWatchlistRequest {
    pub ticker: String,
}

#[derive(Debug, Deserialize)]
pub struct RemoveWatchlistRequest {
    pub ticker: String,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
} 