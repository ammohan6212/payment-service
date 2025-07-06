use axum::{extract::State, Json};
use axum::http::StatusCode;
use sqlx::{PgPool, query, query_as};

use crate::models::cart_item::{CartItem, CartItemRecord};
use crate::utils::response::ApiResponse;

pub async fn process_payment(
    State(pool): State<PgPool>,
    Json(payload): Json<Vec<CartItem>>,
) -> Result<Json<ApiResponse>, (StatusCode, String)> {
    for item in &payload {
        query(
            r#"
            INSERT INTO orders (
                username,
                item_id,
                item_name,
                price,
                quantity,
                image_url,
                payment_method,
                total,
                seller_name,
                address
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8,$9,$10)
            "#
        )
        .bind(&item.username)
        .bind(&item.item_id)
        .bind(&item.item_name)
        .bind(item.price)
        .bind(item.quantity)
        .bind(&item.image_url)
        .bind(&item.payment_method)
        .bind(item.total)
        .bind(&item.seller_name)
        .bind(&item.address)
        .execute(&pool)
        .await
        .map_err(|e| {
            eprintln!("❌ DB insert error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;
    }

    Ok(Json(ApiResponse {
        success: true,
        message: "✅ Payment processed successfully".into(),
    }))
}

pub async fn get_all_payments(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<CartItemRecord>>, (StatusCode, String)> {
    let rows = query_as::<_, CartItemRecord>(
        r#"
        SELECT
            username,
            item_id,
            item_name,
            price,
            quantity,
            image_url,
            payment_method,
            total,
            seller_name
            address
        FROM orders
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        eprintln!("❌ DB fetch error: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    Ok(Json(rows))
}
