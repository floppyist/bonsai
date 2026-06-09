use std::fs;

use serde::Serialize;
use serde::Deserialize;

#[derive(Deserialize, Serialize, Clone)]
pub struct BonsaiOptions {
    pub general: GeneralOptions,
    pub preprocessing: PreprocessingOptions,
    pub lvm: LVMOptions,
    pub logging: LoggingOptions,
    pub database: DatabaseOptions,
    pub tls: TlsOptions,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GeneralOptions {
}

#[derive(Deserialize, Serialize, Clone)]
pub struct PreprocessingOptions {
    pub debug_dir: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct LVMOptions {
    pub ollama_model: String,
    pub ollama_url: String,
    pub temperature: f32,
    pub top_p: f32,
    pub num_thread: i32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct LoggingOptions {
    pub console: bool,
    pub file: bool,
    pub path: String,
    pub level: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct DatabaseOptions {
    pub user_db_url: String,
    pub user_db_max_pool_connections: u32,
    pub ean_db_url: String,
    pub ean_db_max_pool_connections: u32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct TlsOptions {
    pub cert_path: String,
    pub key_path: String,
}

impl Default for BonsaiOptions {
    fn default() -> Self {
        Self {
            general: GeneralOptions {
            },
            preprocessing: PreprocessingOptions {
                debug_dir: "target/debug_output".to_string(),
            },
            lvm: LVMOptions {
                ollama_model: "qwen2.5vl:3b".to_string(),
                ollama_url: "http://localhost:11434/api/chat".to_string(),
                temperature: 0.0,
                top_p: 0.1,
                num_thread: 8,
            },
            logging: LoggingOptions {
                console: true,
                file: false,
                path: "".to_string(),
                level: "info".to_string(),
            },
            database: DatabaseOptions {
                user_db_url: "sqlite://db/user.db".to_string(),
                user_db_max_pool_connections: 5,
                ean_db_url: "sqlite://db/ean.db".to_string(),
                ean_db_max_pool_connections: 5,
            },
            tls: TlsOptions { 
                cert_path: "certs/server.crt".to_string(),
                key_path: "certs/server.key".to_string(),
            },
        }
    }
}

impl BonsaiOptions {
    pub fn load() -> Self {
        let config_path = "config.toml";

        if let Ok(content) = fs::read_to_string(config_path) {
            toml::from_str(&content).unwrap_or_else(|_| Self::default())
        } else {
            let default_options = Self::default();
            let toml_string = toml::to_string_pretty(&default_options)
                .expect("Failed to serialize default config");

            fs::write(config_path, toml_string).expect("Could not create default config.toml");

            default_options
        }
    }
}

