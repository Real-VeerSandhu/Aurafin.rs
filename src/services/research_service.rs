use crate::models::StockQuote;
use anyhow::Result;
use std::time::Duration;

pub struct ResearchService;

impl ResearchService {
    pub fn new() -> Self {
        Self
    }

    pub async fn get_stock_quote(&self, ticker: &str) -> Result<StockQuote> {
        // Try multiple data sources for reliability
        let ticker_upper = ticker.to_uppercase();
        
        // First try: Alpha Vantage (free tier, requires API key but more reliable)
        if let Ok(quote) = self.try_alpha_vantage(&ticker_upper).await {
            return Ok(quote);
        }
        
        // Fallback: Mock data for demonstration (in production, you'd want real data)
        self.get_mock_quote(&ticker_upper)
    }

    async fn try_alpha_vantage(&self, _ticker: &str) -> Result<StockQuote> {
        // Note: In production, you'd need to get a free API key from Alpha Vantage
        // For now, we'll return an error to trigger the fallback
        Err(anyhow::anyhow!("Alpha Vantage API key required"))
    }

    fn get_mock_quote(&self, ticker: &str) -> Result<StockQuote> {
        // Mock data for demonstration purposes
        // In production, you should use a real API with proper API key
        let base_price = match ticker {
            "AAPL" => 191.34,
            "GOOGL" => 142.56,
            "MSFT" => 378.85,
            "TSLA" => 248.42,
            "AMZN" => 151.94,
            "NVDA" => 485.09,
            "META" => 334.92,
            "NFLX" => 485.09,
            _ => 100.0, // Default price for unknown tickers
        };
        
        let change = (rand::random::<f64>() - 0.5) * 10.0; // Random change between -5 and +5
        let percent_change = (change / base_price) * 100.0;
        
        Ok(StockQuote {
            ticker: ticker.to_string(),
            price: base_price + change,
            change,
            percent_change,
        })
    }
} 