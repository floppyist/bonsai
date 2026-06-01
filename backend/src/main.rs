pub mod config;
pub mod logger;
pub mod ocr;

use std::process::exit;

use axum::Router;
use axum::routing::post;
use axum::extract::DefaultBodyLimit;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Json;

use log::debug;

use log::error;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;

use crate::config::BonsaiOptions;
use crate::logger::setup_logging;

#[tokio::main]
async fn main() {
    let options = BonsaiOptions::load();

    if let Err(e) = setup_logging(&options) {
        eprintln!("Error while initializing logger: {}", e);
    }

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/upload-receipt", post(upload_receipt))
        .layer(cors)
        .layer(DefaultBodyLimit::max(50 * 1024 * 1024));

    match tokio::net::TcpListener::bind("0.0.0.0:3000").await {
        Ok(listener) => {
            debug!("Socket binding successfull.");

            match axum::serve(listener, app).await {
                Ok(_) => {},
                Err(e) => {
                    error!("Error serving backend: {}", e);
                    exit(1);
                }
            }
        },
        Err(e) => {
            error!("Error binding tcp listener: {}", e);
            exit(1);
        }
    }
}

async fn upload_receipt(body_bytes: axum::body::Bytes) -> Result<impl IntoResponse, (StatusCode, String)> {
    debug!("Starting OCR pipeline...");

    match ocr::run_ocr_pipeline(&body_bytes).await {
        Ok(receipt) => Ok(Json(receipt)),
        Err(err) => {
            error!("Pipeline failed: {:?}", err);

            Err((StatusCode::INTERNAL_SERVER_ERROR, format!("OCR Pipeline error: {}", err)))
        }
    }
}
