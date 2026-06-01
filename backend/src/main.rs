mod config;
mod logger;
mod ocr;
mod db;

use std::process::exit;
use std::sync::Arc;

use axum::Router;
use axum::extract::State;
use axum::routing::post;
use axum::extract::DefaultBodyLimit;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Json;

use log::error;
use log::info;
use log::warn;

use tower_http::cors::Any;
use tower_http::cors::CorsLayer;

use crate::config::BonsaiOptions;
use crate::db::establish_database_connection;
use crate::logger::setup_logging;

#[tokio::main]
async fn main() {
    let options = BonsaiOptions::load();

    if let Err(e) = setup_logging(&options) {
        eprintln!("Error while initializing logger: {}", e);
    }

    let shared_state = Arc::new(options);

    if establish_database_connection(&shared_state.clone()).await.is_err() {
        warn!("No database is used!");
    }

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/upload-receipt", post(upload_receipt))
        .layer(cors)
        .layer(DefaultBodyLimit::max(50 * 1024 * 1024))
        .with_state(shared_state);

    match tokio::net::TcpListener::bind("0.0.0.0:3000").await {
        Ok(listener) => {
            info!("bonsai backend runs on '0.0.0.0:3000'");

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

async fn upload_receipt(
    State(config): State<Arc<BonsaiOptions>>, body_bytes: axum::body::Bytes
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match ocr::run_ocr_pipeline(&body_bytes, &config).await {
        Ok(receipt) => Ok(Json(receipt)),
        Err(err) => {
            error!("Pipeline failed: {:?}", err);

            Err((StatusCode::INTERNAL_SERVER_ERROR, format!("OCR Pipeline error: {}", err)))
        }
    }
}
