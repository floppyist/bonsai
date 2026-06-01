use serde::Deserialize;
use serde::Serialize;

use anyhow::Context;
use anyhow::Result;

use opencv::imgcodecs;

pub mod preprocessing;
pub mod vlm;

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    #[serde(rename = "name", alias = "name")]
    pub name: String,
    #[serde(rename = "menge", alias = "quantity")]
    pub quantity: i32,
    #[serde(rename = "preis_einzel", alias = "price_unit")]
    pub price_unit: f64,
    #[serde(rename = "preis_gesamt", alias = "price_total")]
    pub price_total: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Receipt {
    #[serde(rename = "haendler", alias = "retailer")]
    pub retailer: String,
    #[serde(rename = "artikel", alias = "articles")]
    pub articles: Vec<Article>,
    #[serde(rename = "gesamtbetrag", alias = "total_amount")]
    pub total_amount: f64,
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

pub async fn run_ocr_pipeline(image_bytes: &[u8]) -> Result<Receipt> {
    let vec_bytes = opencv::core::Vector::<u8>::from_iter(image_bytes.to_vec());
    let img_bgr = imgcodecs::imdecode(&vec_bytes, imgcodecs::IMREAD_COLOR)
        .context("Failed to decode image bytes into OpenCV Mat")?;

    let b64_image = preprocessing::process_image_pipeline_from_mat(&img_bgr)?;
    let vlm_json_output = vlm::run_vlm_analysis(&b64_image).await?;

    let receipt: Receipt = serde_json::from_str(&vlm_json_output)
        .context(format!("Failed to parse VLM JSON output: {}", vlm_json_output))?;

    Ok(receipt)
}
