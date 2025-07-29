# Finance REST API in Rust

A high-performance REST API for finance applications built with Rust, featuring stock price tracking, portfolio management, and market data.

## Features

- **Stock Price Tracking**: Get real-time stock prices and historical data
- **Portfolio Management**: Track your investments with gain/loss calculations
- **Watchlist**: Monitor stocks you're interested in
- **Market Summary**: Overview of market performance with top gainers/losers
- **High Performance**: Built with Rust and Axum for maximum performance
- **Type Safety**: Full type safety with Serde serialization
- **Error Handling**: Comprehensive error handling and validation

## Dependencies

This project uses the following popular and safe Rust packages:

- **Axum**: Modern web framework for Rust
- **Tokio**: Async runtime
- **Serde**: Serialization/deserialization
- **Reqwest**: HTTP client (for future real API integration)
- **Chrono**: Date and time handling
- **UUID**: Unique identifier generation
- **Tracing**: Logging and observability

## Setup

### Prerequisites

- Rust (latest stable version)
- Cargo

### Installation

1. Clone the repository:
```bash
git clone <your-repo-url>
cd rust-backend
```

2. Build the project:
```bash
cargo build
```

3. Run the API:
```bash
cargo run
```

The API will be available at `http://localhost:3000`

## 📚 API Endpoints

### Health Check
```
GET /
```
Returns API status and version information.

### Stock Price
```
GET /api/v1/stocks/{symbol}
```
Get current stock price and market data for a given symbol.

**Example:**
```bash
curl http://localhost:3000/api/v1/stocks/AAPL
```

### Stock History
```
GET /api/v1/stocks/{symbol}/history?days={days}
```
Get historical stock data for a given symbol and time period.

**Parameters:**
- `days` (optional): Number of days of history (default: 30, max: 365)

**Example:**
```bash
curl "http://localhost:3000/api/v1/stocks/AAPL/history?days=7"
```

### Portfolio Management

#### Get Portfolio
```
GET /api/v1/portfolio
```
Get all portfolio items with current values and gain/loss calculations.

#### Add to Portfolio
```
POST /api/v1/portfolio
```
Add a stock to your portfolio.

**Request Body:**
```json
{
  "symbol": "AAPL",
  "shares": 10,
  "price_per_share": 150.0
}
```

**Example:**
```bash
curl -X POST http://localhost:3000/api/v1/portfolio \
  -H "Content-Type: application/json" \
  -d '{"symbol": "AAPL", "shares": 10, "price_per_share": 150.0}'
```

### Watchlist Management

#### Get Watchlist
```
GET /api/v1/watchlist
```
Get all stocks in your watchlist.

#### Add to Watchlist
```
POST /api/v1/watchlist
```
Add a stock to your watchlist.

**Request Body:**
```json
{
  "symbol": "GOOGL"
}
```

**Example:**
```bash
curl -X POST http://localhost:3000/api/v1/watchlist \
  -H "Content-Type: application/json" \
  -d '{"symbol": "GOOGL"}'
```

### Market Summary
```
GET /api/v1/market/summary
```
Get market overview including top gainers, losers, and most active stocks.

## 🧪 Testing

### Using the Bash Script
```bash
./test_api.sh
```

### Using the Python Script
```bash
python3 test_api.py
```

**Requirements for Python script:**
```bash
pip install requests
```

### Manual Testing with curl

1. **Health Check:**
```bash
curl http://localhost:3000/
```

2. **Get Stock Price:**
```bash
curl http://localhost:3000/api/v1/stocks/AAPL
```

3. **Get Stock History:**
```bash
curl "http://localhost:3000/api/v1/stocks/AAPL/history?days=7"
```

4. **Add to Portfolio:**
```bash
curl -X POST http://localhost:3000/api/v1/portfolio \
  -H "Content-Type: application/json" \
  -d '{"symbol": "AAPL", "shares": 10, "price_per_share": 150.0}'
```

5. **Get Portfolio:**
```bash
curl http://localhost:3000/api/v1/portfolio
```

6. **Add to Watchlist:**
```bash
curl -X POST http://localhost:3000/api/v1/watchlist \
  -H "Content-Type: application/json" \
  -d '{"symbol": "GOOGL"}'
```

7. **Get Watchlist:**
```bash
curl http://localhost:3000/api/v1/watchlist
```

8. **Get Market Summary:**
```bash
curl http://localhost:3000/api/v1/market/summary
```

## 📊 Response Format

All API responses follow a consistent format:

```json
{
  "success": true,
  "data": {
    // Response data here
  },
  "message": null,
  "timestamp": "2024-01-01T12:00:00Z"
}
```

Error responses:
```json
{
  "success": false,
  "error": "Error message here",
  "timestamp": "2024-01-01T12:00:00Z"
}
```

## Configuration

The API can be configured using environment variables:

- `RUST_LOG`: Log level (default: "info")
- `PORT`: Server port (default: 3000)

Create a `.env` file for custom configuration:
```env
RUST_LOG=debug
PORT=3000
```

## Project Structure

```
src/
├── main.rs          # Application entry point
├── models.rs        # Data structures and serialization
├── handlers.rs      # HTTP request handlers
├── services.rs      # Business logic layer
└── utils.rs         # Utility functions
```

## Production Considerations

For production deployment, consider:

1. **Database Integration**: Replace in-memory storage with PostgreSQL or similar
2. **Real Stock APIs**: Integrate with Alpha Vantage, Yahoo Finance, or IEX Cloud
3. **Authentication**: Add JWT or OAuth authentication
4. **Rate Limiting**: Implement proper rate limiting
5. **Caching**: Add Redis for caching frequently accessed data
6. **Monitoring**: Add metrics and health checks
7. **Docker**: Containerize the application

## Notes

- This is a prototype using mock data
- In a real application, you would connect to actual stock market APIs
- The current implementation uses in-memory storage (data is lost on restart)
- All stock prices are simulated for demonstration purposes

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

This project is licensed under the MIT License. 