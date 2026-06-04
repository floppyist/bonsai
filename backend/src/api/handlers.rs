use std::sync::Arc;

use axum::Json;
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

    if save_receipt(&state.db, &receipt).await.is_err() {
        error!("Failed to save receipt to database");

        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to persist data".into()));
    }

    Ok(Json(receipt))
}
