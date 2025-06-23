mod handlers;
mod models;
mod utils;

use axum::{
    routing::{get, post},
    Router,
    serve::serve, // ✅ Import serve explicitly from axum::serve
};
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Initialize tracing/logging
    tracing_subscriber::fmt::init();

    // Read DATABASE_URL from env
    let database_url = env::var("DATABASE_URL")
        .expect("❌ DATABASE_URL must be set in environment");

    tracing::info!("📦 Connecting to PostgreSQL database...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("❌ Failed to connect to the database");
    tracing::info!("✅ Connected to database");

    // Define application routes
    let app = Router::new()
        .route("/payment", post(handlers::payment::process_payment))
        .route("/payments", get(handlers::payment::get_all_payments))
        .with_state(pool);

    // Determine the port to listen on
    let port = env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8081);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    tracing::info!("🚀 Starting server at http://{}", addr);

    // Bind and serve the application
    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}
