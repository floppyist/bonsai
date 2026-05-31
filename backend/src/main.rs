use std::fs;

use axum::Router;
use axum::routing::post;
use axum::extract::DefaultBodyLimit;
use axum::extract::Multipart;
use axum::http::StatusCode;
use axum::response::Json;
use axum::response::IntoResponse;

use serde_json::Value;
use std::process::Command;
use tower_http::cors::CorsLayer;
use tower_http::cors::Any;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/upload-receipt", post(upload_receipt))
        .layer(cors)
        .layer(DefaultBodyLimit::max(50 * 1024 * 1024)); /* increase upload size to 50MB */

    println!("Backend runs on http://0.0.0.0:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn upload_receipt(mut multipart: Multipart) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut file_path = String::new();

    while let Some(field) = multipart.next_field().await.map_err(|e| (StatusCode::BAD_REQUEST, format!("Multipart-Error: {}", e)))? {
        if field.name() == Some("image") {
            let data = field.bytes().await.map_err(|e| (StatusCode::BAD_REQUEST, format!("Error while reading bytes: {}", e)))?;
            let file_ext = "jpg"; 

            file_path = format!("/tmp/receipt_{}.{}", Uuid::new_v4(), file_ext);

            fs::write(&file_path, &data).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Write error in /tmp: {}", e)))?;

            break;
        }
    }

    if file_path.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "No image found in request body".to_string()));
    }

    println!("Start VLM OCR for {}...", file_path);

    let output = Command::new("python3")
        .arg("ocr.py")
        .arg(&file_path)
        .output()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Couldn't start python process: {}", e)))?;

    // Remove temporary file
    let _ = fs::remove_file(&file_path);

    let stdout_str = String::from_utf8_lossy(&output.stdout);
    let stderr_str = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Python script crashed: {}", stderr_str)));
    }

    // Find JSON borders
    let json_start = stdout_str.find('{');
    let json_end = stdout_str.rfind('}');

    if let (Some(start), Some(end)) = (json_start, json_end) {
        let raw_json = &stdout_str[start..=end];
        
        match serde_json::from_str::<Value>(raw_json) {
            Ok(parsed_json) => Ok(Json(parsed_json)),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Serde parsing error: {}. Output was: {}", e, stdout_str))),
        }
    } else {
        Err((StatusCode::INTERNAL_SERVER_ERROR, format!("No JSON-Block found in python output. Output was: {}", stdout_str)))
    }
}
