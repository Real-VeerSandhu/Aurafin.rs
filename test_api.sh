#!/bin/bash

echo "🚀 Testing Rust Finance API"
echo "=========================="

# Test Research Module
echo -e "\n📊 Testing Research Module:"
echo "Getting AAPL quote..."
curl -s http://localhost:3000/research/quote/AAPL | jq '.'

echo -e "\nGetting GOOGL quote..."
curl -s http://localhost:3000/research/quote/GOOGL | jq '.'

# Test Portfolio Module
echo -e "\n💼 Testing Portfolio Module:"
echo "Getting current portfolio..."
curl -s http://localhost:3000/portfolio | jq '.'

echo -e "\nAdding MSFT position..."
curl -s -X POST -H "Content-Type: application/json" \
  -d '{"ticker":"MSFT","shares":5}' \
  http://localhost:3000/portfolio/add | jq '.'

echo -e "\nAdding more AAPL shares..."
curl -s -X POST -H "Content-Type: application/json" \
  -d '{"ticker":"AAPL","shares":5}' \
  http://localhost:3000/portfolio/add | jq '.'

echo -e "\nGetting updated portfolio..."
curl -s http://localhost:3000/portfolio | jq '.'

# Test Watchlist Module
echo -e "\n👁️ Testing Watchlist Module:"
echo "Getting current watchlist..."
curl -s http://localhost:3000/watchlist | jq '.'

echo -e "\nAdding GOOGL to watchlist..."
curl -s -X POST -H "Content-Type: application/json" \
  -d '{"ticker":"GOOGL"}' \
  http://localhost:3000/watchlist/add | jq '.'

echo -e "\nAdding NVDA to watchlist..."
curl -s -X POST -H "Content-Type: application/json" \
  -d '{"ticker":"NVDA"}' \
  http://localhost:3000/watchlist/add | jq '.'

echo -e "\nGetting updated watchlist..."
curl -s http://localhost:3000/watchlist | jq '.'

echo -e "\n✅ API testing completed!" 