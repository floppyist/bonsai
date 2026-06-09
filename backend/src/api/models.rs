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
    pub quantity: f64,
    pub price: f64,
    pub ean: Option<String>,
    #[serde(default)]
    pub category: ProductCategory,
    pub brand: String,
}
