use axum::{extract::State, Json};
use axum::http::StatusCode;
use sqlx::PgPool;
use sqlx::query; 

use crate::models::cart_item::{CartItem, CartItemRecord};
use crate::utils::response::ApiResponse;

pub async fn process_payment(
    State(pool): State<PgPool>,
    Json(payload): Json<Vec<CartItem>>,
) -> Result<Json<ApiResponse>, (StatusCode, String)> {
    for item in &payload {
        sqlx::query!(
            r#"
            INSERT INTO orders (
                username,
                item_id,
                item_name,
                price,
                quantity,
                image_url,
                payment_method,
                total
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            item.username,
            item.item_id,
            item.item_name,
            item.price,
            item.quantity,
            item.image_url,
            item.payment_method,
            item.total
        )
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }

    Ok(Json(ApiResponse {
        success: true,
        message: "✅ Payment processed successfully".into(),
    }))
}

pub async fn get_all_payments(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<CartItemRecord>>, (StatusCode, String)> {
    let rows = sqlx::query_as!(
        CartItemRecord,
        r#"
        SELECT
            username,
            item_id,
            item_name,
            price,
            quantity,
            image_url,
            payment_method,
            total
        FROM orders
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(rows))
}
