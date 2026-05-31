use axum::{
    extract::{Multipart, DefaultBodyLimit}, // DefaultBodyLimit importiert
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::post,
    Router,
};
use serde_json::Value;
use std::process::Command;
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;
use std::fs;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/upload-receipt", post(upload_receipt))
        .layer(cors)
        // WICHTIG: Setzt das globale Limit für Requests auf 50 Megabyte hoch
        .layer(DefaultBodyLimit::max(50 * 1024 * 1024)); 

    println!("🚀 Backend läuft auf http://0.0.0.0:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn upload_receipt(mut multipart: Multipart) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut file_path = String::new();

    while let Some(field) = multipart.next_field().await.map_err(|e| (StatusCode::BAD_REQUEST, format!("Multipart-Fehler: {}", e)))? {
        if field.name() == Some("image") {
            let data = field.bytes().await.map_err(|e| (StatusCode::BAD_REQUEST, format!("Fehler beim Lesen der Bytes: {}", e)))?;
            
            let file_ext = "jpg"; 
            file_path = format!("/tmp/receipt_{}.{}", Uuid::new_v4(), file_ext);
            fs::write(&file_path, &data).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Schreibfehler in /tmp: {}", e)))?;
            break;
        }
    }

    if file_path.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Kein Bild im Request-Body gefunden".to_string()));
    }

    println!("Führe OCR-Skript für {} aus...", file_path);
    let output = Command::new("python3")
        .arg("ocr.py")
        // Falls ocr.py nicht im CWD des Backend-Prozesses liegt, hier absoluten Pfad eintragen:
        // .arg("/home/floppyist/workspace/bon_ocr_prototypes/ocr.py")
        .arg(&file_path)
        .output()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Konnte Python-Prozess nicht starten: {}", e)))?;

    // Temporäre Datei löschen
    let _ = fs::remove_file(&file_path);

    let stdout_str = String::from_utf8_lossy(&output.stdout);
    let stderr_str = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Python-Skript abgestürzt: {}", stderr_str)));
    }

    // JSON-Grenzen im stdout finden
    let json_start = stdout_str.find('{');
    let json_end = stdout_str.rfind('}');

    if let (Some(start), Some(end)) = (json_start, json_end) {
        let raw_json = &stdout_str[start..=end];
        
        match serde_json::from_str::<Value>(raw_json) {
            Ok(parsed_json) => Ok(Json(parsed_json)),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Serde Parsing-Fehler: {}. Output war: {}", e, stdout_str))),
        }
    } else {
        Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Kein JSON-Block im Python-Output gefunden. Output war: {}", stdout_str)))
    }
}
