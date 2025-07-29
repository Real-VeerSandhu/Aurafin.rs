use crate::models::*;

pub fn validate_stock_symbol(symbol: &str) -> bool {
    // Basic validation for stock symbols
    // In a real application, you might want to check against a list of valid symbols
    !symbol.trim().is_empty() && symbol.len() <= 10 && symbol.chars().all(|c| c.is_alphanumeric())
}

pub fn format_currency(amount: f64) -> String {
    format!("${:.2}", amount)
}

pub fn format_percentage(value: f64) -> String {
    format!("{:.2}%", value)
}

pub fn calculate_total_portfolio_value(portfolio: &[PortfolioItem]) -> f64 {
    portfolio.iter().map(|item| item.total_value).sum()
}

pub fn calculate_total_gain_loss(portfolio: &[PortfolioItem]) -> f64 {
    portfolio.iter().map(|item| item.gain_loss).sum()
}

pub fn calculate_total_gain_loss_percent(portfolio: &[PortfolioItem]) -> f64 {
    let total_invested: f64 = portfolio.iter().map(|item| item.shares * item.average_price).sum();
    if total_invested > 0.0 {
        calculate_total_gain_loss(portfolio) / total_invested * 100.0
    } else {
        0.0
    }
} 