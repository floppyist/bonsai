mod config;
mod logger;
mod ocr;
mod db;
mod api;

use std::net::SocketAddr;
use std::process::exit;
use std::sync::Arc;

use axum::Router;
use axum::routing::post;
use axum::extract::DefaultBodyLimit;

use axum_server::tls_rustls::RustlsConfig;
use log::error;
use log::info;

use tower_http::cors::Any;
use tower_http::cors::CorsLayer;

use sqlx::SqlitePool;

use crate::config::BonsaiOptions;
use crate::db::establish_database_connection;
use crate::logger::setup_logging;

pub struct AppState {
    pub config: BonsaiOptions,
    pub db: SqlitePool,
}

#[tokio::main]
async fn main() {
    let options = BonsaiOptions::load();

    if let Err(e) = setup_logging(&options) {
        eprintln!("Error while initializing logger: {}", e);
    }

    let pool = match establish_database_connection(&options).await {
        Ok(p) => p,
        Err(_) => {
            error!("Database could not be initialized. Exiting.");
            exit(1);
        }
    };

    let cert_path = options.tls.cert_path.clone();
    let key_path = options.tls.key_path.clone();

    let shared_state = Arc::new(AppState {
        config: options,
        db: pool,
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/upload-receipt", post(api::handlers::upload_receipt))
        .layer(cors)
        .layer(DefaultBodyLimit::max(50 * 1024 * 1024))
        .with_state(shared_state);


    let tls_config = match RustlsConfig::from_pem_file(&cert_path, &key_path).await {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to load TLS certificates: {}", e);
            exit(1);
        }
    };

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    /* FIXME: This is just for testing and will be dynamic in future releases */
    info!("bonsai backend runs on 'https://bonsai.pi:3000'");

    match axum_server::bind_rustls(addr, tls_config)
        .serve(app.into_make_service())
        .await
    {
        Ok(_) => {},
        Err(e) => {
            error!("Error serving backend: '{}':", e);
            exit(1);
        }
    }
}
