#!/usr/bin/env python3
"""
Finance API Test Script
Tests all endpoints of the Rust Finance API
"""

import requests
import json
import time
from typing import Dict, Any

BASE_URL = "http://localhost:3000"

def make_request(method: str, endpoint: str, data: Dict[str, Any] = None) -> Dict[str, Any]:
    """Make a request to the API and return the response"""
    url = f"{BASE_URL}{endpoint}"
    
    try:
        if method.upper() == "GET":
            response = requests.get(url, timeout=10)
        elif method.upper() == "POST":
            response = requests.post(url, json=data, timeout=10)
        else:
            raise ValueError(f"Unsupported method: {method}")
        
        response.raise_for_status()
        return response.json()
    
    except requests.exceptions.RequestException as e:
        print(f"❌ Error making {method} request to {endpoint}: {e}")
        return {"error": str(e)}

def print_response(title: str, response: Dict[str, Any]):
    """Print a formatted response"""
    print(f"\n{'='*50}")
    print(f"📋 {title}")
    print(f"{'='*50}")
    print(json.dumps(response, indent=2))

def test_health_check():
    """Test the health check endpoint"""
    response = make_request("GET", "/")
    print_response("Health Check", response)

def test_stock_price(symbol: str):
    """Test getting stock price"""
    response = make_request("GET", f"/api/v1/stocks/{symbol}")
    print_response(f"Stock Price - {symbol}", response)

def test_stock_history(symbol: str, days: int = 7):
    """Test getting stock history"""
    response = make_request("GET", f"/api/v1/stocks/{symbol}/history?days={days}")
    print_response(f"Stock History - {symbol} ({days} days)", response)

def test_market_summary():
    """Test getting market summary"""
    response = make_request("GET", "/api/v1/market/summary")
    print_response("Market Summary", response)

def test_add_to_portfolio(symbol: str, shares: float, price_per_share: float):
    """Test adding to portfolio"""
    data = {
        "symbol": symbol,
        "shares": shares,
        "price_per_share": price_per_share
    }
    response = make_request("POST", "/api/v1/portfolio", data)
    print_response(f"Add to Portfolio - {symbol}", response)

def test_get_portfolio():
    """Test getting portfolio"""
    response = make_request("GET", "/api/v1/portfolio")
    print_response("Get Portfolio", response)

def test_add_to_watchlist(symbol: str):
    """Test adding to watchlist"""
    data = {"symbol": symbol}
    response = make_request("POST", "/api/v1/watchlist", data)
    print_response(f"Add to Watchlist - {symbol}", response)

def test_get_watchlist():
    """Test getting watchlist"""
    response = make_request("GET", "/api/v1/watchlist")
    print_response("Get Watchlist", response)

def main():
    """Run all tests"""
    print("🧪 Finance API Testing Suite")
    print("=" * 50)
    
    # Test 1: Health Check
    test_health_check()
    
    # Test 2: Stock Prices
    for symbol in ["AAPL", "GOOGL", "MSFT", "TSLA"]:
        test_stock_price(symbol)
        time.sleep(0.5)  # Small delay between requests
    
    # Test 3: Stock History
    test_stock_history("AAPL", 7)
    
    # Test 4: Market Summary
    test_market_summary()
    
    # Test 5: Portfolio Management
    test_add_to_portfolio("AAPL", 10, 150.0)
    test_add_to_portfolio("GOOGL", 5, 2800.0)
    test_get_portfolio()
    
    # Test 6: Watchlist Management
    test_add_to_watchlist("MSFT")
    test_add_to_watchlist("TSLA")
    test_get_watchlist()
    
    print(f"\n{'='*50}")
    print("✅ All tests completed!")
    print("Note: This API uses mock data for demonstration purposes.")
    print("In a real application, you would connect to actual stock market APIs.")

if __name__ == "__main__":
    main() 