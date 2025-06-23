mod handlers;
mod models;
mod utils;

use axum::{routing::{get, post}, Router};
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let database_url = env::var("DATABASE_URL").expect("❌ DATABASE_URL must be set in environment");

    tracing::info!("📦 Connecting to PostgreSQL database...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("❌ Failed to connect to the database");

    let app = Router::new()
        .route("/payment", post(handlers::payment::process_payment))
        .route("/payments", get(handlers::payment::get_all_payments))
        .with_state(pool);

    let port = env::var("PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(8081);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("🚀 Starting server at http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}
