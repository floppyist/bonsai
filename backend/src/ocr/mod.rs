mod preprocessing;
mod vlm;

use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;

use anyhow::Context;
use anyhow::Result;

use log::debug;
use log::info;

use opencv::imgcodecs;

use crate::config::BonsaiOptions;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Confidence {
    High,
    Medium,
    Low,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Field<T> {
    pub value: T,
    pub confidence: Confidence,
}

fn deserialize_flexible_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrFloat {
        Float(f64),
        String(String),
    }

    match StringOrFloat::deserialize(deserializer)? {
        StringOrFloat::Float(f) => Ok(f),
        StringOrFloat::String(s) => s
            .replace(',', ".")
            .trim()
            .parse::<f64>()
            .map_err(serde::de::Error::custom),
    }
}

fn deserialize_field_f64<'de, D>(deserializer: D) -> Result<Field<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct RawField {
        #[serde(deserialize_with = "deserialize_flexible_f64")]
        value: f64,
        confidence: Confidence,
    }

    let raw = RawField::deserialize(deserializer)?;
    Ok(Field {
        value: raw.value,
        confidence: raw.confidence,
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    pub name: String,
    pub quantity: i32,
    #[serde(deserialize_with = "deserialize_flexible_f64")]
    pub price_unit: f64,
    #[serde(deserialize_with = "deserialize_flexible_f64")]
    pub price_total: f64,
    pub confidence: Confidence,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Receipt {
    pub retailer: Field<String>,
    pub date: Field<Option<String>>,
    pub articles: Vec<Article>,
    #[serde(deserialize_with = "deserialize_field_f64")]
    pub total_amount: Field<f64>,
    pub global_confidence: Confidence,
}

#[derive(Serialize)]
struct OllamaOptions {
    temperature: f32,
    top_p: f32,
    num_thread: i32,
}

#[derive(Serialize)]
struct OllamaMessage {
    role: String,
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    images: Option<Vec<String>>,
}

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    messages: Vec<OllamaMessage>,
    options: OllamaOptions,
    format: String,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}

pub async fn run_ocr_pipeline(image_bytes: &[u8], config: &BonsaiOptions) -> Result<Receipt> {
    info!("Starting OCR pipeline...");

    let vec_bytes = opencv::core::Vector::<u8>::from_iter(image_bytes.to_vec());
    let img_bgr = imgcodecs::imdecode(&vec_bytes, imgcodecs::IMREAD_COLOR)
        .context("Failed to decode image bytes into OpenCV Mat")?;

    let b64_image = preprocessing::process_image_pipeline_from_mat(&img_bgr, config)?;
    let vlm_json_output = vlm::run_vlm_analysis(&b64_image, config).await?;

    let receipt: Receipt = serde_json::from_str(&vlm_json_output)
        .context(format!("Failed to parse VLM JSON output: {}", vlm_json_output))?;

    debug!("Output: {:?}", receipt);
    info!("OCR finished");

    Ok(receipt)
}
