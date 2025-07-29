use crate::models::*;
use chrono::{Duration, Utc};
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

// In-memory storage for demo purposes
// In a real application, you'd use a database
lazy_static::lazy_static! {
    static ref PORTFOLIO: Mutex<HashMap<String, PortfolioItem>> = Mutex::new(HashMap::new());
    static ref WATCHLIST: Mutex<HashMap<String, WatchlistItem>> = Mutex::new(HashMap::new());
}

pub struct FinanceService;

impl FinanceService {
    pub fn get_stock_price(symbol: &str) -> Result<StockPrice, String> {
        // Mock stock data - in a real app, you'd call a real API like Alpha Vantage, Yahoo Finance, etc.
        let base_price = match symbol.to_uppercase().as_str() {
            "AAPL" => 150.0,
            "GOOGL" => 2800.0,
            "MSFT" => 300.0,
            "TSLA" => 800.0,
            "AMZN" => 3300.0,
            "NVDA" => 500.0,
            "META" => 350.0,
            "NFLX" => 600.0,
            _ => 100.0, // Default price for unknown symbols
        };

        // Add some realistic price variation
        let change = (rand::random::<f64>() - 0.5) * 10.0;
        let price = base_price + change;
        let change_percent = (change / base_price) * 100.0;
        let volume = rand::random::<u64>() % 10000000 + 1000000;

        Ok(StockPrice {
            symbol: symbol.to_uppercase(),
            price,
            change,
            change_percent,
            volume,
            market_cap: Some(volume * 100),
            high_24h: Some(price + rand::random::<f64>() * 5.0),
            low_24h: Some(price - rand::random::<f64>() * 5.0),
            open: Some(base_price),
            previous_close: Some(base_price - change),
            timestamp: Utc::now(),
        })
    }

    pub fn get_stock_history(symbol: &str, days: u32) -> Result<StockHistory, String> {
        let mut data = Vec::new();
        let base_price = match symbol.to_uppercase().as_str() {
            "AAPL" => 150.0,
            "GOOGL" => 2800.0,
            "MSFT" => 300.0,
            "TSLA" => 800.0,
            "AMZN" => 3300.0,
            "NVDA" => 500.0,
            "META" => 350.0,
            "NFLX" => 600.0,
            _ => 100.0,
        };

        for i in 0..days {
            let date = Utc::now() - Duration::days(i as i64);
            let open = base_price + (rand::random::<f64>() - 0.5) * 20.0;
            let high = open + rand::random::<f64>() * 10.0;
            let low = open - rand::random::<f64>() * 10.0;
            let close = low + rand::random::<f64>() * (high - low);
            let volume = rand::random::<u64>() % 10000000 + 1000000;

            data.push(HistoricalDataPoint {
                date,
                open,
                high,
                low,
                close,
                volume,
            });
        }

        data.reverse(); // Put in chronological order

        Ok(StockHistory {
            symbol: symbol.to_uppercase(),
            data,
        })
    }

    pub fn add_to_portfolio(request: AddToPortfolioRequest) -> Result<PortfolioItem, String> {
        let current_price = Self::get_stock_price(&request.symbol)?.price;
        
        let portfolio_item = PortfolioItem {
            id: Uuid::new_v4(),
            symbol: request.symbol.to_uppercase(),
            shares: request.shares,
            average_price: request.price_per_share,
            current_price,
            total_value: request.shares * current_price,
            gain_loss: request.shares * (current_price - request.price_per_share),
            gain_loss_percent: ((current_price - request.price_per_share) / request.price_per_share) * 100.0,
            added_at: Utc::now(),
        };

        PORTFOLIO.lock().unwrap().insert(request.symbol.to_uppercase(), portfolio_item.clone());
        Ok(portfolio_item)
    }

    pub fn get_portfolio() -> Result<Vec<PortfolioItem>, String> {
        let portfolio = PORTFOLIO.lock().unwrap();
        let mut items: Vec<PortfolioItem> = portfolio.values().cloned().collect();
        
        // Update current prices and calculations
        for item in &mut items {
            if let Ok(stock_price) = Self::get_stock_price(&item.symbol) {
                item.current_price = stock_price.price;
                item.total_value = item.shares * item.current_price;
                item.gain_loss = item.shares * (item.current_price - item.average_price);
                item.gain_loss_percent = ((item.current_price - item.average_price) / item.average_price) * 100.0;
            }
        }

        Ok(items)
    }

    pub fn add_to_watchlist(request: AddToWatchlistRequest) -> Result<WatchlistItem, String> {
        let stock_price = Self::get_stock_price(&request.symbol)?;
        
        let watchlist_item = WatchlistItem {
            id: Uuid::new_v4(),
            symbol: request.symbol.to_uppercase(),
            current_price: stock_price.price,
            change: stock_price.change,
            change_percent: stock_price.change_percent,
            added_at: Utc::now(),
        };

        WATCHLIST.lock().unwrap().insert(request.symbol.to_uppercase(), watchlist_item.clone());
        Ok(watchlist_item)
    }

    pub fn get_watchlist() -> Result<Vec<WatchlistItem>, String> {
        let watchlist = WATCHLIST.lock().unwrap();
        let mut items: Vec<WatchlistItem> = watchlist.values().cloned().collect();
        
        // Update current prices
        for item in &mut items {
            if let Ok(stock_price) = Self::get_stock_price(&item.symbol) {
                item.current_price = stock_price.price;
                item.change = stock_price.change;
                item.change_percent = stock_price.change_percent;
            }
        }

        Ok(items)
    }

    pub fn get_market_summary() -> Result<MarketSummary, String> {
        let popular_symbols = vec!["AAPL", "GOOGL", "MSFT", "TSLA", "AMZN", "NVDA", "META", "NFLX"];
        let mut all_stocks = Vec::new();
        let mut total_market_cap = 0u64;
        let mut total_volume = 0u64;

        for symbol in &popular_symbols {
            if let Ok(stock) = Self::get_stock_price(symbol) {
                total_market_cap += stock.market_cap.unwrap_or(0);
                total_volume += stock.volume;
                all_stocks.push(stock);
            }
        }

        // Sort by gain/loss for top gainers and losers
        all_stocks.sort_by(|a, b| b.change_percent.partial_cmp(&a.change_percent).unwrap());
        let gainers = all_stocks.iter().take(5).cloned().collect();
        let losers = all_stocks.iter().rev().take(5).cloned().collect();

        // Sort by volume for most active
        all_stocks.sort_by(|a, b| b.volume.cmp(&a.volume));
        let most_active = all_stocks.iter().take(5).cloned().collect();

        Ok(MarketSummary {
            total_market_cap,
            total_volume,
            gainers,
            losers,
            most_active,
            timestamp: Utc::now(),
        })
    }
} 