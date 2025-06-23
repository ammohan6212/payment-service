use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize)]
pub struct CartItem {
    pub username: String,
    pub item_id: String,
    pub item_name: String,
    pub price: f64,
    pub quantity: i32,
    pub image_url: String,
    pub payment_method: String,
    pub total: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct CartItemRecord {
    pub username: String,
    pub item_id: String,
    pub item_name: String,
    pub price: f64,
    pub quantity: i32,
    pub image_url: String,
    pub payment_method: String,
    pub total: f64,
}
