use std::sync::Arc;

use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use axum::extract::Multipart;
use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use axum::http::StatusCode;

use axum_extra::extract::cookie::{Cookie, CookieJar};

use sqlx::Row;

use crate::AppState;
use crate::api::models::AuthPayload;
use crate::auth::crypto::{hash_password, verify_password, generate_jwt};
use crate::auth::extractor::AuthenticatedUser;
use crate::db;

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AuthPayload>,
) -> impl IntoResponse {
    let username = payload.username.trim().to_string();

    if username.len() < 3 || username.len() > 32 {
        return (StatusCode::BAD_REQUEST, "Invalid username (3-32 characters)").into_response();
    }
    if payload.password.len() < 8 {
        return (StatusCode::BAD_REQUEST, "Password too short (min 8 characters)").into_response();
    }
    if payload.password.len() > 128 {
        return (StatusCode::BAD_REQUEST, "Password too long (max 128 characters)").into_response();
    }

    let hashed_pw = match hash_password(&payload.password) {
        Ok(h) => h,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Registration failed").into_response(),
    };

    if db::create_user(&state.user_db, &username, &hashed_pw).await.is_err() {
        return (StatusCode::CONFLICT, "Registration failed").into_response();
    }

    StatusCode::CREATED.into_response()
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AuthPayload>,
) -> impl IntoResponse {
    let user = match db::get_user_by_username(&state.user_db, &payload.username).await {
        Ok(Some(u)) => u,
        _ => return (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response(),
    };

    if !verify_password(&payload.password, &user.1).unwrap_or(false) {
        return (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response();
    }

    let token = match generate_jwt(user.0, &state.jwt_secret) {
        Ok(t) => t,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Token generation failed").into_response(),
    };

    let cookie = Cookie::build(("bonsai_session", token))
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(axum_extra::extract::cookie::SameSite::Strict)
        .build();

    (CookieJar::new().add(cookie), StatusCode::OK).into_response()
}

pub async fn me(
    State(state): State<Arc<AppState>>,
    user: AuthenticatedUser,
) -> impl IntoResponse {
    let row = sqlx::query("SELECT username, profile_picture_url FROM users WHERE id = ?")
        .bind(user.user_id)
        .fetch_optional(&state.user_db)
    .await;

    match row {
        Ok(Some(r)) => {
            let username: String = r.get("username");
            let profile_picture_url: Option<String> = r.get("profile_picture_url");

            (StatusCode::OK, Json(serde_json::json!({
                "user_id": user.user_id,
                "username": username,
                "profile_picture_url": profile_picture_url
            }))).into_response()
        }
        _ => (StatusCode::UNAUTHORIZED, "User not found").into_response(),
    }
}

pub async fn logout() -> impl IntoResponse {
    let cookie = Cookie::build(("bonsai_session", ""))
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(axum_extra::extract::cookie::SameSite::Strict)
        .build();

    (CookieJar::new().add(cookie), StatusCode::OK).into_response()
}

pub async fn upload_avatar(
    State(state): State<Arc<AppState>>,
    user: AuthenticatedUser,
    mut multipart: Multipart,
) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        if field.name() == Some("avatar") {
            let content_type = field.content_type().unwrap_or("image/jpeg");

            let ext = match content_type {
                "image/png" => "png",
                "image/webp" => "webp",
                _ => "jpg", // Fallback
            };

            let filename = format!("avatar_{}.{}", user.user_id, ext);
            let filepath = format!("./storage/avatars/{}", filename);

            let data = match field.bytes().await {
                // Prevent too large image uploads
                Ok(b) if b.len() > 2 * 1024 * 1024 => {
                    return (StatusCode::PAYLOAD_TOO_LARGE, "Avatar image is too large (max 2 MB)").into_response();
                },
                Ok(b) => b,
                Err(_) => return (StatusCode::BAD_REQUEST, "Failed to read image data").into_response(),
            };

            // Validate image content via magic bytes (JPEG, PNG, WebP) for security reasons
            if !data.starts_with(b"\xff\xd8\xff")
            && !data.starts_with(b"\x89PNG")
            && !data.starts_with(b"RIFF")
            {
                return (StatusCode::BAD_REQUEST, "Invalid image format").into_response();
            }

            let mut file = match File::create(&filepath).await {
                Ok(f) => f,
                Err(e) => {
                    log::error!("Failed to create file: {}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to save image").into_response();
                }
            };

            if file.write_all(&data).await.is_err() {
                return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to write image").into_response();
            }

            let avatar_url = format!("/avatars/{}", filename);

            if let Err(e) = sqlx::query("UPDATE users SET profile_picture_url = ? WHERE id = ?")
                .bind(&avatar_url)
                .bind(user.user_id)
                .execute(&state.user_db)
            .await
            {
                log::error!("Failed to update avatar in DB: {}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, "Database update failed").into_response();
            }

            return (StatusCode::OK, Json(serde_json::json!({ "avatar_url": avatar_url }))).into_response();
        }
    }

    (StatusCode::BAD_REQUEST, "No avatar field found").into_response()
}
