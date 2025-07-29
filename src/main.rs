use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod handlers;
mod models;
mod services;
mod utils;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenv::dotenv().ok();

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build our application with a route
    let app = Router::new()
        .route("/", get(handlers::health_check))
        .route("/api/v1/stocks/:symbol", get(handlers::get_stock_price))
        .route("/api/v1/stocks/:symbol/history", get(handlers::get_stock_history))
        .route("/api/v1/portfolio", get(handlers::get_portfolio))
        .route("/api/v1/portfolio", post(handlers::add_to_portfolio))
        .route("/api/v1/watchlist", get(handlers::get_watchlist))
        .route("/api/v1/watchlist", post(handlers::add_to_watchlist))
        .route("/api/v1/market/summary", get(handlers::get_market_summary))
        .layer(cors);

    // Run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Finance API listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
} 