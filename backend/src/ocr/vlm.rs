use anyhow::Result;
use anyhow::Context;
use log::debug;

use crate::config::BonsaiOptions;
use crate::ocr::OllamaMessage;
use crate::ocr::OllamaOptions;
use crate::ocr::OllamaRequest;
use crate::ocr::OllamaResponse;

pub async fn run_vlm_analysis(b64_image: &str, config: &BonsaiOptions) -> Result<String> {
    let model = &config.lvm.ollama_model;
    let url = &config.lvm.ollama_url;

    debug!("Forwarding request to Ollama model: '{}'...", model);

    let prompt = "Extract data from this German receipt precisely into the requested JSON format.\n\n\
                  Take note of these store-specific patterns if present:\n\
                  - NETTO: Item quantities are located ABOVE the item row: [quantity] x [unit_price] \\n [name] [price_total]\n\
                  - REWE: Item quantities are located BELOW the item row\n\
                  - LIDL: Item row structure looks like: [name] [unit_price] x [quantity] [price_total]\n\n\
                  Expected JSON schema:\n\
                  {\n\
                    \"retailer\": \"Name of the store\",\n\
                    \"date\": \"Date of the receipt\",\n\
                    \"articles\": [\n\
                      {\"name\": \"Cleaned item name\", \"quantity\": 1, \"price_unit\": 0.00, \"price_total\": 0.00}\n\
                    ],\n\
                    \"total_amount\": 0.00\n\
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
            temperature: 0.0,
            top_p: 0.1,
            num_thread: 8,
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
