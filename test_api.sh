#!/bin/bash

# Finance API Test Script
# Make sure the API is running on http://localhost:3000

BASE_URL="http://127.0.0.1:3000"

echo "🧪 Testing Finance API"
echo "======================"

# Test health check
echo -e "\n1. Testing Health Check"
echo "GET $BASE_URL/"
curl -s "$BASE_URL/" | jq '.'

# Test stock price
echo -e "\n\n2. Testing Stock Price (AAPL)"
echo "GET $BASE_URL/api/v1/stocks/AAPL"
curl -s "$BASE_URL/api/v1/stocks/AAPL" | jq '.'

# Test stock history
echo -e "\n\n3. Testing Stock History (AAPL, 7 days)"
echo "GET $BASE_URL/api/v1/stocks/AAPL/history?days=7"
curl -s "$BASE_URL/api/v1/stocks/AAPL/history?days=7" | jq '.'

# Test market summary
echo -e "\n\n4. Testing Market Summary"
echo "GET $BASE_URL/api/v1/market/summary"
curl -s "$BASE_URL/api/v1/market/summary" | jq '.'

# Test adding to portfolio
echo -e "\n\n5. Testing Add to Portfolio"
echo "POST $BASE_URL/api/v1/portfolio"
curl -s -X POST "$BASE_URL/api/v1/portfolio" \
  -H "Content-Type: application/json" \
  -d '{
    "symbol": "AAPL",
    "shares": 10,
    "price_per_share": 150.0
  }' | jq '.'

# Test getting portfolio
echo -e "\n\n6. Testing Get Portfolio"
echo "GET $BASE_URL/api/v1/portfolio"
curl -s "$BASE_URL/api/v1/portfolio" | jq '.'

# Test adding to watchlist
echo -e "\n\n7. Testing Add to Watchlist"
echo "POST $BASE_URL/api/v1/watchlist"
curl -s -X POST "$BASE_URL/api/v1/watchlist" \
  -H "Content-Type: application/json" \
  -d '{
    "symbol": "GOOGL"
  }' | jq '.'

# Test getting watchlist
echo -e "\n\n8. Testing Get Watchlist"
echo "GET $BASE_URL/api/v1/watchlist"
curl -s "$BASE_URL/api/v1/watchlist" | jq '.'

# Test multiple stock prices
echo -e "\n\n9. Testing Multiple Stock Prices"
for symbol in "MSFT" "TSLA" "AMZN" "NVDA"; do
  echo "GET $BASE_URL/api/v1/stocks/$symbol"
  curl -s "$BASE_URL/api/v1/stocks/$symbol" | jq '.data.symbol, .data.price, .data.change_percent'
done

echo -e "\n\n✅ API Testing Complete!"
echo "Note: This uses mock data. In a real application, you would connect to actual stock APIs." 