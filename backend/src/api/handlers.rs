use sqlx::Row;
use std::sync::Arc;

use axum::Json;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::http::StatusCode;
use axum::extract::State;
use axum::body::Bytes;

use log::error;

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
