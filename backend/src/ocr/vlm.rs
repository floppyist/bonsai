use anyhow::Result;
use anyhow::Context;
use log::error;
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

    let prompt = "[Task: Extract German receipt data into strict JSON]\n\
        [Constraint 1: If NO valid date found -> date=null. Do NOT infer.]\n\
        [Constraint 2: EXTREMELY IMPORTANT - For article names, COPY the EXACT raw text from the receipt. Do NOT fix typos, do NOT change casing, do NOT normalize abbreviations. Leave the text absolutely raw.]\n\n\
        [Store_Layout_Patterns]:\n\
        - NETTO: [qty] x [unit_p] \\n [name] [total_p] (Qty ABOVE item)\n\
        - REWE:  [name] [total_p] \\n [qty] x [unit_p] (Qty BELOW item)\n\
        - EDEKA: [name] [unit_p] € x [qty] [total_p]   (Qty RIGHT of item)\n\
        - LIDL:  [name] [unit_p] x [qty] [total_p]     (Qty RIGHT of item)\n\n\
        [Output_Schema]:\n\
        {\n\
        \"retailer\": { \"value\": string },\n\
        \"date\": { \"value\": \"YYYY-MM-DD\"|null },\n\
        \"articles\": [\n\
        { \"name\": string, \"quantity\": int, \"price_unit\": float, \"price_total\": float }\n\
        ],\n\
        \"total_amount\": { \"value\": float }\n\
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

    let status = res.status();

    let response_text = res
        .text()
        .await
        .context("Failed to read response body as text from Ollama")?;

    if !status.is_success() {
        error!("Ollama API returned error status {}: {}", status, response_text);

        return Err(anyhow::anyhow!("Ollama error (Status {}): {}", status, response_text));
    }

    let response_json: OllamaResponse = serde_json::from_str(&response_text)
        .with_context(|| format!(
            "Failed to deserialize JSON response into OllamaResponse. Raw body was: '{}'", 
            response_text
        ))?;

    Ok(response_json.message.content)
}
