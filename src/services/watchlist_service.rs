use crate::models::{Watchlist, AddWatchlistRequest, RemoveWatchlistRequest};
use crate::services::research_service::ResearchService;
use anyhow::Result;
use serde_json;
use std::fs;
use std::path::Path;

pub struct WatchlistService {
    file_path: String,
    research_service: ResearchService,
}

impl WatchlistService {
    pub fn new() -> Self {
        Self {
            file_path: "data/watchlist.json".to_string(),
            research_service: ResearchService::new(),
        }
    }

    fn ensure_data_dir(&self) -> Result<()> {
        let data_dir = Path::new("data");
        if !data_dir.exists() {
            fs::create_dir_all(data_dir)?;
        }
        Ok(())
    }

    fn load_watchlist(&self) -> Result<Watchlist> {
        self.ensure_data_dir()?;
        
        if !Path::new(&self.file_path).exists() {
            return Ok(Watchlist { tickers: Vec::new() });
        }

        let content = fs::read_to_string(&self.file_path)?;
        let watchlist: Watchlist = serde_json::from_str(&content)?;
        Ok(watchlist)
    }

    fn save_watchlist(&self, watchlist: &Watchlist) -> Result<()> {
        self.ensure_data_dir()?;
        let content = serde_json::to_string_pretty(watchlist)?;
        fs::write(&self.file_path, content)?;
        Ok(())
    }

    pub fn get_watchlist(&self) -> Result<Watchlist> {
        self.load_watchlist()
    }

    pub async fn add_ticker(&self, request: AddWatchlistRequest) -> Result<Watchlist> {
        // Validate ticker exists by trying to get a quote
        self.research_service.get_stock_quote(&request.ticker).await?;
        
        let mut watchlist = self.load_watchlist()?;
        
        // Check for duplicates
        if watchlist.tickers.contains(&request.ticker) {
            return Err(anyhow::anyhow!("Ticker already in watchlist"));
        }
        
        watchlist.tickers.push(request.ticker);
        self.save_watchlist(&watchlist)?;
        Ok(watchlist)
    }

    pub fn remove_ticker(&self, request: RemoveWatchlistRequest) -> Result<Watchlist> {
        let mut watchlist = self.load_watchlist()?;
        
        if !watchlist.tickers.contains(&request.ticker) {
            return Err(anyhow::anyhow!("Ticker not found in watchlist"));
        }
        
        watchlist.tickers.retain(|t| t != &request.ticker);
        self.save_watchlist(&watchlist)?;
        Ok(watchlist)
    }
} 