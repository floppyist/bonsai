use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FinalizeReceiptPayload {
    pub retailer: String,
    pub receipt_date: String,
    pub total_amount: f64,
    pub articles: Vec<FinalizeArticleItem>,
}

#[derive(Debug, Deserialize)]
pub struct FinalizeArticleItem {
    pub name: String,
    pub quantity: f64,
    pub price: f64,
    pub ean: Option<String>,
}
