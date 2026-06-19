use std::time::Duration;
use std::time::UNIX_EPOCH;
use std::time::SystemTime;

use log::error;
use serde::Serialize;
use serde::Deserialize;

use argon2::PasswordVerifier;
use argon2::PasswordHasher;
use argon2::Argon2;
use argon2::password_hash::SaltString;
use argon2::password_hash::PasswordHash;
use argon2::password_hash::rand_core::OsRng;

use jsonwebtoken::EncodingKey;
use jsonwebtoken::Header;
use jsonwebtoken::encode;

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

pub fn generate_jwt(user_id: i32, secret: &[u8]) -> Result<String, String> {
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

    let jwt_secret = EncodingKey::from_secret(secret);

    encode(&Header::default(), &claims, &jwt_secret)
        .map_err(|e| {
            error!("Failed to encode JWT token: {}", e);
            "Internal server error".to_string()
        })
}
