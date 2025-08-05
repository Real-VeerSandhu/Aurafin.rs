# Rust Finance API

A lightweight, barebones Rust API for managing a personal finance portfolio with stock research, portfolio tracking, and watchlist functionality.

## Features

- **Research Module**: Get real-time stock quotes using Yahoo Finance API
- **Portfolio Module**: Track owned positions (ticker + shares)
- **Watchlist Module**: Monitor stocks without owning them
- **JSON File Storage**: Simple, file-based data persistence
- **RESTful API**: Clean HTTP endpoints for all operations

## Tech Stack

- **Language**: Rust
- **Web Framework**: Axum
- **HTTP Client**: Reqwest
- **Serialization**: Serde
- **Stock Data**: Yahoo Finance API (free, no API key required)
- **Storage**: JSON files in `data/` directory

## Quick Start

### Prerequisites

- Rust (latest stable version)
- Cargo

### Installation

1. Clone the repository
2. Build the project:
   ```bash
   cargo build
   ```

3. Run the server:
   ```bash
   cargo run
   ```

The API will be available at `http://localhost:3000`

## API Endpoints

### Research Module

#### Get Stock Quote
```
GET /research/quote/{ticker}
```

**Example:**
```bash
curl http://localhost:3000/research/quote/AAPL
```

**Response:**
```json
{
  "success": true,
  "data": {
    "ticker": "AAPL",
    "price": 191.34,
    "change": -1.23,
    "percent_change": -0.64
  },
  "error": null
}
```

### Portfolio Module

#### Get Portfolio
```
GET /portfolio
```

#### Add Position
```
POST /portfolio/add
Content-Type: application/json

{
  "ticker": "AAPL",
  "shares": 10
}
```

#### Remove Position
```
POST /portfolio/remove
Content-Type: application/json

{
  "ticker": "AAPL",
  "shares": 5
}
```

### Watchlist Module

#### Get Watchlist
```
GET /watchlist
```

#### Add Ticker to Watchlist
```
POST /watchlist/add
Content-Type: application/json

{
  "ticker": "TSLA"
}
```

#### Remove Ticker from Watchlist
```
POST /watchlist/remove
Content-Type: application/json

{
  "ticker": "TSLA"
}
```

## Data Storage

The API uses JSON files for data persistence:

- `data/portfolio.json` - Stores portfolio positions
- `data/watchlist.json` - Stores watchlist tickers

Files are automatically created when needed.

## Stock Data Source

This API currently uses **mock data** for demonstration purposes. For production use, you have several options:

### Recommended: Alpha Vantage (Free Tier)
- **Cost**: Free (500 API calls per day)
- **Setup**: Get API key from [Alpha Vantage](https://www.alphavantage.co/support/#api-key)
- **Implementation**: Update `try_alpha_vantage()` method in `src/services/research_service.rs`

### Alternative Data Sources

1. **Yahoo Finance**: Free but rate-limited, requires proper user agent
2. **Finnhub**: Free tier available, real-time data
3. **IEX Cloud**: Free tier available, reliable data
4. **Polygon.io**: Free tier available, comprehensive data

### Setting Up Real Data

To use real stock data:

1. Get an API key from your chosen provider
2. Update the `ResearchService` in `src/services/research_service.rs`
3. Replace the mock data implementation with real API calls

Example Alpha Vantage implementation:
```rust
async fn try_alpha_vantage(&self, ticker: &str) -> Result<StockQuote> {
    let api_key = std::env::var("ALPHA_VANTAGE_API_KEY")?;
    let url = format!(
        "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol={}&apikey={}",
        ticker, api_key
    );
    
    let response = self.client.get(&url).send().await?;
    let data: Value = response.json().await?;
    
    // Parse Alpha Vantage response...
    // Implementation details depend on the API response format
}
```

## Project Structure

```
src/
├── main.rs              # Server entry point
├── models/              # Data structures
│   └── mod.rs
├── routes/              # HTTP route handlers
│   ├── mod.rs
│   ├── research.rs
│   ├── portfolio.rs
│   └── watchlist.rs
└── services/            # Business logic
    ├── mod.rs
    ├── research_service.rs
    ├── portfolio_service.rs
    └── watchlist_service.rs
```

## Development

### Running in Development Mode

```bash
RUST_LOG=debug cargo run
```

### Building for Production

```bash
cargo build --release
```

### Testing

```bash
cargo test
```

### API Testing

Run the provided test script to verify all endpoints:

```bash
# Make sure the server is running first
cargo run &

# Run the test script
./test_api.sh
```

The test script will:
- Test stock quote retrieval
- Test portfolio management (add/remove positions)
- Test watchlist management (add/remove tickers)
- Verify JSON file persistence

## Security Considerations

- **Input Validation**: All ticker symbols are validated against real stock data
- **Error Handling**: Comprehensive error handling with meaningful messages
- **CORS**: Permissive CORS enabled for development (configure for production)
- **File Permissions**: Ensure `data/` directory has appropriate permissions

## Production Deployment

For production use, consider:

1. **Database**: Replace JSON files with PostgreSQL or SQLite
2. **Authentication**: Add user authentication and authorization
3. **Rate Limiting**: Implement API rate limiting
4. **CORS**: Configure CORS for your domain
5. **Logging**: Set up proper logging infrastructure
6. **Monitoring**: Add health checks and monitoring

## License

MIT License - feel free to use and modify as needed. 