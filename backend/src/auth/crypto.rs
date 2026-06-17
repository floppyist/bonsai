use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, SaltString},
    Argon2, PasswordHasher, PasswordVerifier,
};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Deserialize, Serialize};
use log::error;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

pub const JWT_SECRET: &[u8] = b"super_geheimes_bonsai_secret_123456";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,  // user_id
    pub exp: i64,  // expiration timestamp (Unix time)
}

pub fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    
    match Argon2::default().hash_password(password.as_bytes(), &salt) {
        Ok(hashed_password) => Ok(hashed_password.to_string()),
        Err(e) => {
            error!("Failed to hash password: {}", e);
            Err("Internal server error".to_string())
        }
    }
}

pub fn verify_password(password: &str, stored_hash: &str) -> Result<bool, String> {
    let parsed_hash = PasswordHash::new(stored_hash)
        .map_err(|e| {
            error!("Invalid password hash format in database: {}", e);
            "Internal server error".to_string()
        })?;

    match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(argon2::password_hash::Error::Password) => Ok(false),
        Err(e) => {
            error!("Password verification failed: {}", e);
            Err("Internal server error".to_string())
        }
    }
}

pub fn generate_jwt(user_id: i32) -> Result<String, String> {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .map_err(|e| {
            error!("System time is before UNIX EPOCH: {}", e);
            "Internal server error".to_string()
        })?;
    
    let expiration = (since_the_epoch + Duration::from_secs(24 * 60 * 60)).as_secs() as i64;

    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };

    let encoding_key = EncodingKey::from_secret(JWT_SECRET);

    encode(&Header::default(), &claims, &encoding_key)
        .map_err(|e| {
            error!("Failed to encode JWT token: {}", e);
            "Internal server error".to_string()
        })
}
