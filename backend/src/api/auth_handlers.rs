use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use std::sync::Arc;

use crate::AppState;
use crate::api::models::AuthPayload;
use crate::auth::crypto::{hash_password, verify_password, generate_jwt};
use crate::db;

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AuthPayload>,
) -> impl IntoResponse {
    let hashed_pw = match hash_password(&payload.password) {
        Ok(h) => h,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Registration failed").into_response(),
    };

    if let Err(_) = db::create_user(&state.user_db, &payload.username, &hashed_pw).await {
        return (StatusCode::CONFLICT, "User already exists or DB error").into_response();
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

    let token = match generate_jwt(user.0) {
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
