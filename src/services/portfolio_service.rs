use crate::models::{Portfolio, Position, AddPositionRequest, RemovePositionRequest};
use anyhow::Result;
use serde_json;
use std::fs;
use std::path::Path;

pub struct PortfolioService {
    file_path: String,
}

impl PortfolioService {
    pub fn new() -> Self {
        Self {
            file_path: "data/portfolio.json".to_string(),
        }
    }

    fn ensure_data_dir(&self) -> Result<()> {
        let data_dir = Path::new("data");
        if !data_dir.exists() {
            fs::create_dir_all(data_dir)?;
        }
        Ok(())
    }

    fn load_portfolio(&self) -> Result<Portfolio> {
        self.ensure_data_dir()?;
        
        if !Path::new(&self.file_path).exists() {
            return Ok(Portfolio { positions: Vec::new() });
        }

        let content = fs::read_to_string(&self.file_path)?;
        let portfolio: Portfolio = serde_json::from_str(&content)?;
        Ok(portfolio)
    }

    fn save_portfolio(&self, portfolio: &Portfolio) -> Result<()> {
        self.ensure_data_dir()?;
        let content = serde_json::to_string_pretty(portfolio)?;
        fs::write(&self.file_path, content)?;
        Ok(())
    }

    pub fn get_portfolio(&self) -> Result<Portfolio> {
        self.load_portfolio()
    }

    pub fn add_position(&self, request: AddPositionRequest) -> Result<Portfolio> {
        let mut portfolio = self.load_portfolio()?;
        
        // Find existing position or create new one
        if let Some(position) = portfolio.positions.iter_mut().find(|p| p.ticker == request.ticker) {
            position.shares += request.shares;
        } else {
            portfolio.positions.push(Position {
                ticker: request.ticker,
                shares: request.shares,
            });
        }

        self.save_portfolio(&portfolio)?;
        Ok(portfolio)
    }

    pub fn remove_position(&self, request: RemovePositionRequest) -> Result<Portfolio> {
        let mut portfolio = self.load_portfolio()?;
        
        if let Some(position) = portfolio.positions.iter_mut().find(|p| p.ticker == request.ticker) {
            if position.shares < request.shares {
                return Err(anyhow::anyhow!("Not enough shares to remove"));
            }
            
            position.shares -= request.shares;
            
            // Remove position if shares become 0
            if position.shares == 0 {
                portfolio.positions.retain(|p| p.ticker != request.ticker);
            }
        } else {
            return Err(anyhow::anyhow!("Position not found"));
        }

        self.save_portfolio(&portfolio)?;
        Ok(portfolio)
    }
} 