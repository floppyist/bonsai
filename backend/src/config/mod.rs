use std::fs;

use serde::Serialize;
use serde::Deserialize;

#[derive(Deserialize, Serialize)]
pub struct BonsaiOptions {
    pub general: GeneralOptions,
    pub preprocessing: PreprocessingOptions,
    pub lvm: LVMOptions,
    pub logging: LoggingOptions,
}

#[derive(Deserialize, Serialize)]
pub struct GeneralOptions {
}

#[derive(Deserialize, Serialize)]
pub struct PreprocessingOptions {
    debug_dir: String,
}

#[derive(Deserialize, Serialize)]
pub struct LVMOptions {
    ollama_model: String,
    ollama_url: String,
}

#[derive(Deserialize, Serialize)]
pub struct LoggingOptions {
    pub console: bool,
    pub file: bool,
    pub path: String,
    pub level: String,
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
            },
            logging: LoggingOptions {
                console: false,
                file: false,
                path: "".to_string(),
                level: "error".to_string(),
            }

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

