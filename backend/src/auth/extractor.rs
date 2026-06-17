use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{decode, DecodingKey, Validation};
use log::error;

use crate::auth::crypto::{Claims, JWT_SECRET};

pub struct AuthenticatedUser {
    pub user_id: i32,
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_request_parts(parts, state)
            .await
            .map_err(|e| {
                error!("Failed to parse cookie jar: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            })?;

        let cookie = jar.get("bonsai_session")
            .ok_or((StatusCode::UNAUTHORIZED, "Missing session cookie".to_string()))?;

        let user_id = Self::validate_session_token(cookie.value())?;

        Ok(AuthenticatedUser { user_id })
    }
}

impl AuthenticatedUser {
    fn validate_session_token(token: &str) -> Result<i32, (StatusCode, String)> {
        if token.is_empty() {
            return Err((StatusCode::BAD_REQUEST, "Session token is empty".to_string()));
        }

        let decoding_key = DecodingKey::from_secret(JWT_SECRET);

        match decode::<Claims>(token, &decoding_key, &Validation::default()) {
            Ok(token_data) => Ok(token_data.claims.sub),
            Err(e) => {
                error!("JWT validation failed: {}", e);
                Err((StatusCode::UNAUTHORIZED, "Invalid or expired session token".to_string()))
            }
        }
    }
}
