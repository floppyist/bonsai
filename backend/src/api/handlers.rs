use std::sync::Arc;

use sqlx::Row;

use axum::Json;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::http::StatusCode;
use axum::extract::State;
use axum::body::Bytes;

use log::error;

use super::models::FinalizeReceiptPayload;
use crate::api::categories::ProductCategory;

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

    let result = sqlx::query("SELECT name, category, brand FROM ean_products WHERE ean = ?")
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
            let category: String = row.try_get("category").unwrap_or_else(|_| "UNCATEGORIZED".to_string());
            let brand: String = row.try_get("brand").unwrap_or_else(|_| "UNKNOWN".to_string());

            let response = serde_json::json!({
                "ean": ean,
                "name": product_name,
                "category": category,
                "brand": if brand.trim().is_empty() { "UNKNOWN".to_string() } else { brand },
                "found": true
            });
            Ok(Json(response))
        }
        None => {
            let response = serde_json::json!({
                "ean": ean,
                "name": "",
                "category": "UNCATEGORIZED",
                "brand": "UNKNOWN",
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
        let mut existing_category: ProductCategory = ProductCategory::Uncategorized;
        let mut existing_brand: String = "UNKNOWN".to_string();

        if let Some(ref ean_code) = article.ean {
            let row = sqlx::query(
                "SELECT id, ean, category, brand FROM products WHERE ean = ? AND is_deleted = 0 LIMIT 1"
            )
                .bind(ean_code)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            if let Some(r) = row {
                product_id = Some(r.get("id"));
                existing_ean = r.get("ean");
                existing_category = r.get("category");
                existing_brand = r.get("brand");
            }
        }

        if product_id.is_none() {
            let row = sqlx::query(
                "SELECT id, ean, category, brand FROM products WHERE name = ? AND is_deleted = 0 LIMIT 1"
            )
                .bind(&article.name)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            if let Some(r) = row {
                product_id = Some(r.get("id"));
                existing_ean = r.get("ean");
                existing_category = r.get("category");
                existing_brand = r.get("brand");
            }
        }

        let final_product_id = match product_id {
            Some(id) => {
                let need_ean_update = existing_ean.is_none() && article.ean.is_some();
                let need_category_update = existing_category == ProductCategory::Uncategorized && article.category != ProductCategory::Uncategorized;
                
                let clean_existing = existing_brand.trim().to_uppercase().replace('\'', "").replace('"', "");
                let clean_incoming = article.brand.trim().to_uppercase().replace('\'', "").replace('"', "");

                let is_existing_unknown = clean_existing == "UNKNOWN" || clean_existing.is_empty();
                let is_incoming_valid = clean_incoming != "UNKNOWN" && !clean_incoming.is_empty();

                let need_brand_update = is_existing_unknown && is_incoming_valid;

                if need_ean_update || need_category_update || need_brand_update {
                    let target_ean = article.ean.as_ref().or(existing_ean.as_ref());
                    let target_category = if need_category_update { article.category } else { existing_category };
                    
                    let target_brand = if need_brand_update { &article.brand } else { &existing_brand };

                    sqlx::query("UPDATE products SET ean = ?, category = ?, brand = ? WHERE id = ?")
                        .bind(target_ean)
                        .bind(target_category)
                        .bind(target_brand)
                        .bind(id)
                        .execute(&mut *tx)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                }
                id
            },
            None => {
                let new_product_row = sqlx::query(
                    "INSERT INTO products (name, ean, category, brand) VALUES (?, ?, ?, ?) RETURNING id"
                )
                .bind(&article.name)
                .bind(&article.ean)
                .bind(article.category)
                .bind(&article.brand)
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
                    "INSERT OR IGNORE INTO ean_products (ean, name, category, brand) VALUES (?, ?, ?, ?)" // <-- brand hinzugefügt
                )
                    .bind(ean_code)
                    .bind(&article.name)
                    .bind(article.category)
                    .bind(&article.brand) // <-- NEU gebunden
                    .execute(&state.ean_db)
                .await;
            }
        }
    }

    tx.commit().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::CREATED)
}
