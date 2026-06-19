mod api;
mod config;
mod db;
mod logger;
mod ocr;
mod util;
mod auth;

use std::fs;
use std::net::SocketAddr;
use std::process::exit;
use std::sync::Arc;

use axum::routing::get;
use axum::Router;
use axum::extract::DefaultBodyLimit;
use axum::routing::post;
use axum_server::tls_rustls::RustlsConfig;

use log::error;
use log::info;

use tower_http::cors::AllowOrigin;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

use sqlx::SqlitePool;

use crate::config::BonsaiOptions;

pub struct AppState {
    pub config: config::BonsaiOptions,
    pub user_db: SqlitePool,
    pub ean_db: SqlitePool,
    pub http_client: reqwest::Client,
    pub jwt_secret: Vec<u8>,
}

#[tokio::main]
async fn main() {
    let options = BonsaiOptions::load();

    // Setup basic paths
    fs::create_dir_all("certs").ok();
    fs::create_dir_all("db").ok();
    fs::create_dir_all("storage/avatars").ok();

    let cert_path = options.tls.cert_path.clone();
    let key_path = options.tls.key_path.clone();

    if let Err(e) = logger::setup_logging(&options) {
        eprintln!("Error while initializing logger: {}", e);
    }

    let pools = match db::establish_database_connections(&options).await {
        Ok(p) => p,
        Err(_) => {
            error!("Database could not be initialized. Exiting.");

            exit(1);
        }
    };

    let jwt_secret= match std::env::var("BONSAI_JWT_SECRET") {
        Ok(secret) => secret.into_bytes(),
        Err(_) => {
            error!("BONSAI_JWT_SECRET environment variable is not set! Abort.");
            exit(1);
        }
    };

    let shared_state = Arc::new(AppState {
        config: options,
        user_db: pools.user_db,
        ean_db: pools.ean_db,
        http_client: reqwest::Client::new(),
        jwt_secret,
    });

    let ip = shared_state.config.general.ip;
    let port = shared_state.config.general.port;

    let cors_origins = if shared_state.config.general.allowed_origins.contains(&"*".to_string()) {
        AllowOrigin::any()
    } else {
        AllowOrigin::list(
            shared_state.config.general.allowed_origins.iter().map(|o| o.parse().unwrap())
        )
    };

    let cors = CorsLayer::new()
        .allow_origin(cors_origins)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/upload-receipt", post(api::handlers::upload_receipt))
        .route(
            "/api/receipt/revalidate",
            post(api::handlers::revalidate_articles),
        )
        .route("/api/ean/{ean}", get(api::handlers::lookup_ean))
        .route(
            "/api/finalize-receipt",
            post(api::handlers::finalize_receipt),
        )
        .route(
            "/api/pantry/summary",
            get(api::handlers::get_pantry_summary),
        )
        .route("/api/pantry/items", get(api::handlers::get_pantry_items))
        .route("/api/receipts", get(api::handlers::get_receipts_history))
        .route(
            "/api/receipts/{id}",
            get(api::handlers::get_receipt_details),
        )
        .route("/api/health", get(|| async { "OK" }))
        .route("/api/auth/register", post(api::auth_handlers::register))
        .route("/api/auth/login", post(api::auth_handlers::login))
        .route("/api/auth/me", get(api::auth_handlers::me))
        .route("/api/auth/logout", post(api::auth_handlers::logout))
        .route("/api/auth/upload-avatar", post(api::auth_handlers::upload_avatar))
        .nest_service("/avatars", ServeDir::new("./storage/avatars"))
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

    let addr = SocketAddr::from((ip, port));

    info!("bonsai backend runs on 'https://{}'", addr);

    match axum_server::bind_rustls(addr, tls_config)
        .serve(app.into_make_service())
    .await
    {
        Ok(_) => {}
        Err(e) => {
            error!("Error serving backend: '{}':", e);

            exit(1);
        }
    }
}
