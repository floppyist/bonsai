use anyhow::Result;
use anyhow::Context;
use log::info;

use crate::config::BonsaiOptions;
use crate::ocr::OllamaMessage;
use crate::ocr::OllamaOptions;
use crate::ocr::OllamaRequest;
use crate::ocr::OllamaResponse;

pub async fn run_vlm_analysis(b64_image: &str, config: &BonsaiOptions) -> Result<String> {
    let model = &config.lvm.ollama_model;
    let url = &config.lvm.ollama_url;

    info!("Forwarding request to Ollama model: '{}'...", model);

    let prompt = "Extract data from this German receipt precisely into the requested JSON format.\n\n\
        Strict Rules for Specific Fields:\n\
        - DATE: Extract the date ONLY if a valid date is explicitly found on the receipt. If no valid date is present, set the \"value\" of date to null. Do not guess or infer the date.\n\n\
        Take note of these store-specific item/quantity patterns:\n\
        - NETTO: Item quantities and unit prices are located ABOVE the item row: [quantity] x [unit_price] \\n [name] [price_total]\n\
        - REWE: Item quantities and unit prices are located BELOW the item row\n\
        - EDEKA: Item quantities and unit prices are located to the RIGHT of the item name in the format: [name] [unit_price] € x [quantity] [price_total]\n\
        - LIDL: Item row structure looks like: [name] [unit_price] x [quantity] [price_total]\n\n\
        Expected JSON schema:\n\
        {\n\
        \"retailer\": {\n\
        \"value\": \"Name of the store\",\n\
        \"confidence\": \"high\" or \"medium\" or \"low\"\n\
        },\n\
        \"date\": {\n\
        \"value\": \"DD-MM-YYYY\" or null,\n\
        \"confidence\": \"high\" or \"medium\" or \"low\"\n\
        },\n\
        \"articles\": [\n\
        {\n\
        \"name\": \"Cleaned item name\",\n\
        \"quantity\": 1,\n\
        \"price_unit\": 0.00,\n\
        \"price_total\": 0.00,\n\
        \"confidence\": \"high\" or \"medium\" or \"low\"\n\
        }\n\
        ],\n\
        \"total_amount\": {\n\
        \"value\": 0.00,\n\
        \"confidence\": \"high\" or \"medium\" or \"low\"\n\
        },\n\
        \"global_confidence\": \"high\" or \"medium\" or \"low\"\n\
        }";

    let request_body = OllamaRequest {
        model: model.to_string(),
        messages: vec![
            OllamaMessage {
                role: "system".to_string(),
                content: "You are a precise OCR parser for receipts. Respond exclusively in the requested JSON format without any extra markdown wrapper or text.".to_string(),
                images: None,
            },
            OllamaMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
                images: Some(vec![b64_image.to_string()]),
            },
        ],
        options: OllamaOptions {
            temperature: config.lvm.temperature,
            top_p: config.lvm.top_p,
            num_thread: config.lvm.num_thread,
        },
        format: "json".to_string(),
        stream: false,
    };

    let client = reqwest::Client::new();

    let res = client.post(url)
        .json(&request_body)
        .send()
        .await
        .context("Failed to send HTTP request to Ollama endpoint")?;

    let response_json: OllamaResponse = res
        .json()
        .await
        .context("Failed to deserialize JSON response body from Ollama")?;

    Ok(response_json.message.content)
}
