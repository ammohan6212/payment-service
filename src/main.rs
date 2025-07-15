mod handlers;
mod models;
mod utils;

use axum::{
    routing::{get, post},
    Router,
    serve::serve,
};
use sqlx::postgres::{PgPoolOptions, PgConnectOptions}; // Import PgConnectOptions
use std::env;
use std::net::SocketAddr;
use std::str::FromStr; // Needed for PgConnectOptions::from_str
use std::time::Duration;
use tokio::net::TcpListener;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let database_url = env::var("DATABASE_URL")
        .expect("❌ DATABASE_URL must be set in environment");

    tracing::info!("📦 Connecting to PostgreSQL database...");

    // 1. Parse the DATABASE_URL into PgConnectOptions
    let connect_options = PgConnectOptions::from_str(&database_url)
        .expect("Invalid DATABASE_URL")
        .connect_timeout(Duration::from_secs(5)); // Set connect_timeout here

    // 2. Use connect_with and the configured PgConnectOptions
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .acquire_timeout(Duration::from_secs(5)) // Timeout to get a connection from the pool
        // Removed .connect_timeout() from here as it's now on connect_options
        // Optional: Keep a minimum number of connections alive
        // .min_connections(5)
        // Optional: Disconnect idle connections after a certain time
        // .idle_timeout(Duration::from_secs(300)) // 5 minutes
        .connect_with(connect_options) // Use connect_with and the pre-configured options
        .await
        .expect("❌ Failed to connect to the database");
    tracing::info!("✅ Connected to database");

    // ✅ Create table if it doesn't exist
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS orders (
        id SERIAL PRIMARY KEY,
        username TEXT NOT NULL,
        item_id TEXT NOT NULL,
        item_name TEXT NOT NULL,
        price NUMERIC NOT NULL,
        quantity INTEGER NOT NULL,
        image_url TEXT,
        payment_method TEXT NOT NULL,
        total NUMERIC NOT NULL,
        seller_name TEXT NOT NULL,
        address TEXT NOT NULL,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
        "#,
    )
    .execute(&pool)
    .await
    .expect("❌ Failed to create orders table");

    // Define application routes
    let app = Router::new()
        .route("/health", get(handlers::health::health_check))
        .route("/payment", post(handlers::payment::process_payment))
        .route("/payments", get(handlers::payment::get_all_payments))
        .with_state(pool);

    let port = env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8081);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    tracing::info!("🚀 Starting server at http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}