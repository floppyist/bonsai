use axum::{
    extract::Multipart,
    http::{StatusCode},
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
    // CORS erlauben, damit die PWA (die oft auf einem anderen Port läuft) zugreifen darf
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/upload-receipt", post(upload_receipt))
        .layer(cors);

    println!("🚀 Backend läuft auf http://0.0.0.0:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn upload_receipt(mut multipart: Multipart) -> Result<impl IntoResponse, (StatusCode, String)> {
    // 1. Datei aus dem Request extrahieren
    let mut file_path = String::new();
    
    while let Some(field) = multipart.next_field().await.map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))? {
        if field.name() == Some("image") {
            let data = field.bytes().await.map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
            
            // Temporäre Datei erstellen
            let file_ext = "jpg"; // Wir gehen von jpg/jpeg aus
            file_path = format!("/tmp/receipt_{}.{}", Uuid::new_v4(), file_ext);
            fs::write(&file_path, &data).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            break;
        }
    }

    if file_path.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Kein Bild gefunden".to_string()));
    }

    // 2. Dein Python-Skript ausführen (ocr.py muss im selben Verzeichnis liegen)
    println!("Führe OCR-Skript für {} aus...", file_path);
    let output = Command::new("python3")
        .arg("ocr.py")
        .arg(&file_path)
        .output()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Fehler beim Ausführen von Python: {}", e)))?;

    // Temp-Datei aufräumen
    let _ = fs::remove_file(&file_path);

    let stdout_str = String::from_utf8_lossy(&output.stdout);
    let stderr_str = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Skript-Fehler: {}", stderr_str)));
    }

    // 3. JSON aus dem stdout extrahieren (ignoriert die [DEBUG] und ✅ Zeilen)
    let json_start = stdout_str.find('{');
    let json_end = stdout_str.rfind('}');

    if let (Some(start), Some(end)) = (json_start, json_end) {
        let raw_json = &stdout_str[start..=end];
        
        // JSON validieren und parsen
        match serde_json::from_str::<Value>(raw_json) {
            Ok(parsed_json) => Ok(Json(parsed_json)),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("JSON Parsing Fehler: {}", e))),
        }
    } else {
        Err((StatusCode::INTERNAL_SERVER_ERROR, "Konnte kein JSON im Output des Skripts finden.".to_string()))
    }
}
