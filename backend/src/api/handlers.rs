use std::sync::Arc;

use sqlx::Row;
use sqlx::SqlitePool;

use axum::Json;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::http::StatusCode;
use axum::extract::State;
use axum::body::Bytes;
use axum::Extension;

use log::error;

use super::models::FinalizeReceiptPayload;

use crate::AppState;
use crate::ocr;
use crate::db::save_receipt;

pub async fn upload_receipt(
    State(state): State<Arc<AppState>>,
    body_bytes: Bytes
) -> Result<impl IntoResponse, (StatusCode, String)> {

    let receipt = match ocr::run_ocr_pipeline(&body_bytes, &state.config).await {
        Ok(r) => r,
        Err(err) => {
            error!("OCR Pipeline failed: {:?}", err);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("OCR Pipeline error: {}", err)));
        }
    };

    if save_receipt(&state.user_db, &receipt).await.is_err() {
        error!("Failed to save receipt to database");

        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to persist data".into()));
    }

    Ok(Json(receipt))
}

pub async fn lookup_ean(
    State(state): State<Arc<AppState>>,
    Path(ean): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {

    let result = sqlx::query("SELECT name FROM ean_products WHERE ean = ?")
        .bind(&ean)
        .fetch_optional(&state.ean_db)
        .await
        .map_err(|e| {
            error!("Database error during EAN lookup for {}: {}", ean, e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal databases error".to_string())
        })?;

    match result {
        Some(row) => {
            let product_name: String = row.get("name");
            let response = serde_json::json!({
                "ean": ean,
                "name": product_name,
                "found": true
            });
            Ok(Json(response))
        }
        None => {
            let response = serde_json::json!({
                "ean": ean,
                "name": "",
                "found": false
            });

            Ok(Json(response))
        }
    }
}

pub async fn finalize_receipt(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<FinalizeReceiptPayload>,
) -> Result<StatusCode, (StatusCode, String)> {

    let user_db = &state.user_db;

    let mut tx = user_db.begin().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let receipt_row = sqlx::query(
        "INSERT INTO receipts (retailer, receipt_date) VALUES (?, ?) RETURNING id"
    )
        .bind(&payload.retailer)
        .bind(&payload.receipt_date)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let receipt_id: i64 = receipt_row.get("id");

for article in payload.articles {
        let mut product_id: Option<i64> = None;
        let mut existing_ean: Option<String> = None;

        if let Some(ref ean_code) = article.ean {
            let row = sqlx::query(
                "SELECT id, ean FROM products WHERE ean = ? AND is_deleted = 0 LIMIT 1"
            )
            .bind(ean_code)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            if let Some(r) = row {
                product_id = Some(r.get("id"));
                existing_ean = r.get("ean");
            }
        }

        if product_id.is_none() {
            let row = sqlx::query(
                "SELECT id, ean FROM products WHERE name = ? AND is_deleted = 0 LIMIT 1"
            )
            .bind(&article.name)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            if let Some(r) = row {
                product_id = Some(r.get("id"));
                existing_ean = r.get("ean");
            }
        }

        let final_product_id = match product_id {
            Some(id) => {
                // Wenn wir eine neue EAN vom User haben, das Produkt in der DB aber noch keine hat: Update!
                if existing_ean.is_none() && article.ean.is_some() {
                    sqlx::query("UPDATE products SET ean = ? WHERE id = ?")
                        .bind(&article.ean)
                        .bind(id)
                        .execute(&mut *tx)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                }
                id
            },
            None => {
                let new_product_row = sqlx::query(
                    "INSERT INTO products (name, ean) VALUES (?, ?) RETURNING id"
                )
                .bind(&article.name)
                .bind(&article.ean)
                .fetch_one(&mut *tx)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

                new_product_row.get("id")
            }
        };

        let price_in_cents = (article.price * 100.0).round() as i64;
        sqlx::query(
            "INSERT INTO receipt_items (receipt_id, product_id, price, quantity) VALUES (?, ?, ?, ?)"
        )
        .bind(receipt_id)
        .bind(final_product_id)
        .bind(price_in_cents)
        .bind(article.quantity)
        .execute(&mut *tx)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        if let Some(ref ean_code) = article.ean {
            if !ean_code.trim().is_empty() {
                let _ = sqlx::query(
                    "INSERT OR IGNORE INTO ean_products (ean, name) VALUES (?, ?)"
                )
                .bind(ean_code)
                .bind(&article.name)
                .execute(&state.ean_db)
                .await;
            }
        }
    }

    tx.commit().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::CREATED)
}
