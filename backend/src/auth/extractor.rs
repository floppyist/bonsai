use std::sync::Arc;

use log::error;

use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::extract::FromRequestParts;
use axum::extract::FromRef;
use axum_extra::extract::cookie::CookieJar;

use jsonwebtoken::Validation;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::decode;

use crate::{AppState, auth::crypto::{Claims}};

pub struct AuthenticatedUser {
    pub user_id: i32,
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    Arc<AppState>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = Arc::<AppState>::from_ref(state);

        let jar = CookieJar::from_request_parts(parts, state)
            .await
            .map_err(|e| {
                error!("Failed to parse cookie jar: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            })?;

        let cookie = jar.get("bonsai_session")
            .ok_or((StatusCode::UNAUTHORIZED, "Missing session cookie".to_string()))?;

        let user_id = Self::validate_session_token(cookie.value(), &app_state.jwt_secret)?;

        Ok(AuthenticatedUser { user_id })
    }
}

impl AuthenticatedUser {
    fn validate_session_token(token: &str, secret: &[u8]) -> Result<i32, (StatusCode, String)> {
        if token.is_empty() {
            return Err((StatusCode::BAD_REQUEST, "Session token is empty".to_string()));
        }

        let decoding_key = DecodingKey::from_secret(secret);

        match decode::<Claims>(token, &decoding_key, &Validation::default()) {
            Ok(token_data) => Ok(token_data.claims.sub),
            Err(e) => {
                error!("JWT validation failed: {}", e);
                Err((StatusCode::UNAUTHORIZED, "Invalid or expired session token".to_string()))
            }
        }
    }
}
