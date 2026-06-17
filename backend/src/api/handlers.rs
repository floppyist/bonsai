use std::sync::Arc;

use axum::Json;
use axum::body::Bytes;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;

use log::error;

use super::models::FinalizeReceiptPayload;
use crate::api::models::RevalidateReceiptPayload;
use crate::auth::extractor::AuthenticatedUser;
use crate::ocr;
use crate::{AppState, util};

pub async fn upload_receipt(
    State(state): State<Arc<AppState>>,
    user: AuthenticatedUser,
    body_bytes: Bytes,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let receipt_struct =
    match ocr::run_ocr_pipeline(&state.http_client, &body_bytes, &state.config).await {
        Ok(r) => r,
        Err(err) => {
            error!("OCR Pipeline failed: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("OCR Pipeline error: {}", err),
            ));
        }
    };

    let mut receipt_val: serde_json::Value = serde_json::to_value(receipt_struct).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to serialize OCR response: {}", e),
        )
    })?;

    let extracted_retailer = receipt_val
        .get("retailer")
        .and_then(|r| {
            if let Some(v) = r.get("value").and_then(|v| v.as_str()) {
                Some(v.to_string())
            } else if let Some(a) = r.get("address").and_then(|a| a.as_str()) {
                Some(a.to_string())
            } else {
                r.as_str().map(|s| s.to_string())
            }
        })
        .unwrap_or_default()
        .trim()
        .to_string();

    if !extracted_retailer.is_empty() {
        let articles = receipt_val
            .get_mut("articles")
            .and_then(|a| a.as_array_mut());

        if let Some(articles_array) = articles {
            for article in articles_array {
                if let Some(raw_name) = article.get("name").and_then(|n| n.as_str()) {
                    let clean_name = util::clean_ocr_name(raw_name);

                    if clean_name.is_empty() {
                        continue;
                    }

                    article["raw_ocr_name"] = serde_json::json!(clean_name);

                    // Execute the clean 3-tier matching layer from the database module
                    let matched_data = crate::db::resolve_article_metadata(
                        &state.ean_db,
                        &state.user_db,
                        &extracted_retailer,
                        &clean_name,
                        user.user_id,
                    ).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

                    if let Some(data) = matched_data {
                        article["name"] = serde_json::json!(data.name);
                        article["category"] = serde_json::json!(data.category);

                        if data.brand != "UNKNOWN" && !data.brand.trim().is_empty() {
                            article["brand"] = serde_json::json!(data.brand);
                        }

                        if let Some(ref ean_code) = data.ean
                        && article.get("ean").and_then(|e| e.as_str()).is_none()
                        {
                            article["ean"] = serde_json::json!(util::sanitize_ean(ean_code));
                        }
                    }
                }
            }
        }
    }

    Ok(Json(receipt_val))
}

pub async fn lookup_ean(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(ean): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let clean_ean = util::sanitize_ean(&ean);

    let db_result = crate::db::fetch_product_by_ean(&state.ean_db, &clean_ean)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal database error".to_string(),
            )
        })?;

    match db_result {
        Some(product) => {
            let response = serde_json::json!({
                "ean": clean_ean,
                "name": product.name,
                "category": product.category,
                "brand": if product.brand.trim().is_empty() { "UNKNOWN".to_string() } else { product.brand },
                "found": true
            });
            Ok(Json(response))
        }
        None => {
            let response = serde_json::json!({
                "ean": clean_ean,
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
    user: AuthenticatedUser,
    Json(payload): Json<FinalizeReceiptPayload>,
) -> Result<StatusCode, (StatusCode, String)> {
    crate::db::save_finalized_receipt(&state.user_db, &state.ean_db, payload, user.user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::CREATED)
}

pub async fn get_pantry_summary(
    State(state): State<Arc<AppState>>,
    user: AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let items = crate::db::fetch_pantry_summary(&state.user_db, user.user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let summary: Vec<serde_json::Value> = items
        .into_iter()
        .map(|item| {
            serde_json::json!({
                "category": item.category,
                "total_quantity": item.total_quantity,
                "total_price": (item.total_price_cents as f64) / 100.0
            })
        })
        .collect();

    Ok(Json(serde_json::json!(summary)))
}

pub async fn get_pantry_items(
    State(state): State<Arc<AppState>>,
    user: AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let items = crate::db::fetch_pantry_items(&state.user_db, user.user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let formatted_items: Vec<serde_json::Value> = items
        .into_iter()
        .map(|item| {
            serde_json::json!({
                "name": item.name,
                "brand": item.brand,
                "category": item.category,
                "quantity": item.quantity,
                "price": item.unit_price * item.quantity
            })
        })
        .collect();

    Ok(Json(serde_json::json!(formatted_items)))
}

pub async fn get_receipts_history(
    State(state): State<Arc<AppState>>,
    user: AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let receipts = crate::db::fetch_receipts_history(&state.user_db, user.user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let formatted: Vec<serde_json::Value> = receipts
        .into_iter()
        .map(|r| {
            serde_json::json!({
                "id": r.id,
                "retailer": r.retailer,
                "date": r.receipt_date.unwrap_or(r.created_at),
                "itemCount": r.item_count,
                "totalAmount": (r.total_price_cents as f64) / 100.0
            })
        })
        .collect();

    Ok(Json(serde_json::json!(formatted)))
}

// FIXME: It would be better to load the receipt history into the frontend all at once rather than loading the details one by one each time the page is opened
pub async fn get_receipt_details(
    State(state): State<Arc<AppState>>,
    user: AuthenticatedUser,
    Path(receipt_id): Path<i64>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let items = crate::db::fetch_receipt_details(&state.user_db, receipt_id, user.user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let formatted: Vec<serde_json::Value> = items
        .into_iter()
        .map(|item| {
            serde_json::json!({
                "name": item.name,
                "brand": item.brand,
                "category": item.category,
                "ean": item.ean,
                "quantity": item.quantity,
                "price": (item.unit_price_cents as f64) / 100.0
            })
        })
        .collect();

    Ok(Json(serde_json::json!(formatted)))
}

pub async fn revalidate_articles(
    State(state): State<Arc<AppState>>,
    user: AuthenticatedUser,
    Json(payload): Json<RevalidateReceiptPayload>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut articles = payload.articles;
    let retailer = payload.retailer.trim();

    if !retailer.is_empty() {
        for article in &mut articles {
            let source_name = if !article.raw_ocr_name.is_empty() {
                &article.raw_ocr_name
            } else {
                &article.name
            };

            let clean_name = util::clean_ocr_name(source_name);

            if clean_name.is_empty() {
                continue;
            }

            // Utilizes the exact same decoupled core cascading database matching pipeline
            let matched_data = crate::db::resolve_article_metadata(
                &state.ean_db,
                &state.user_db,
                retailer,
                &clean_name,
                user.user_id,
            )
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            if let Some(data) = matched_data {
                article.name = data.name;
                article.category = data.category;

                if data.brand != "UNKNOWN" && !data.brand.trim().is_empty() {
                    article.brand = data.brand;
                }

                if article.ean.is_none() {
                    article.ean = data.ean;
                }
            }
        }
    }

    Ok(Json(articles))
}
