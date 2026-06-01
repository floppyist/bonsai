pub mod ocr;

use axum::Router;
use axum::routing::post;
use axum::extract::DefaultBodyLimit;
use axum::http::StatusCode;
use axum::response::{Json, IntoResponse};
use tower_http::cors::{CorsLayer, Any};

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/upload-receipt", post(upload_receipt))
        .layer(cors)
        .layer(DefaultBodyLimit::max(50 * 1024 * 1024));

    println!("Backend running on http://0.0.0.0:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn upload_receipt(body_bytes: axum::body::Bytes) -> Result<impl IntoResponse, (StatusCode, String)> {
    println!("Starting native Rust OCR pipeline...");

    match ocr::run_ocr_pipeline(&body_bytes).await {
        Ok(receipt) => Ok(Json(receipt)),
        Err(err) => {
            eprintln!("[ERROR] Pipeline failed: {:?}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, format!("OCR Pipeline error: {}", err)))
        }
    }
}
