use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize)]
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
