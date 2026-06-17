use serde::Deserialize;

use crate::api::categories::ProductCategory;

#[derive(Debug, Deserialize)]
pub struct FinalizeReceiptPayload {
    pub retailer: String,
    pub receipt_date: String,
    pub articles: Vec<FinalizeArticleItem>,
}

#[derive(Debug, Deserialize)]
pub struct FinalizeArticleItem {
    pub name: String,
    pub raw_ocr_name: String,
    pub quantity: f64,
    pub price: f64,
    pub ean: Option<String>,
    #[serde(default)]
    pub category: ProductCategory,
    pub brand: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct RevalidateReceiptPayload {
    pub retailer: String,
    pub articles: Vec<RevalidateArticleItem>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct RevalidateArticleItem {
    pub name: String,
    #[serde(default)]
    pub raw_ocr_name: String,
    pub quantity: f64,
    pub price: f64,
    pub ean: Option<String>,
    #[serde(default)]
    pub category: ProductCategory,
    pub brand: String,
}

#[derive(serde::Deserialize)]
pub struct AuthPayload {
    pub username: String,
    pub password: String,
}
