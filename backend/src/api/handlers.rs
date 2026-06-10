use std::sync::Arc;
use sqlx::Row;
use axum::Json;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::http::StatusCode;
use axum::extract::State;
use axum::body::Bytes;
use log::error;

use super::models::FinalizeReceiptPayload;
use crate::api::categories::ProductCategory;
use crate::AppState;
use crate::api::models::RevalidateReceiptPayload;
use crate::ocr;

fn sanitize_ean(ean: &str) -> String {
    ean.trim()
        .trim_matches('\'')
        .trim_matches('"')
        .to_string()
}

fn clean_ocr_name(raw_name: &str) -> String {
    let intermediate = raw_name
        .to_uppercase()
        .replace('Ä', "AE")
        .replace('Ö', "OE")
        .replace('Ü', "UE")
        .replace('ß', "SS")
        .chars()
        .filter(|c| c.is_alphanumeric() || [' ', '-', '&', '.', ',', '%'].contains(c))
        .collect::<String>();

    // Wirft alle mehrfachen Leerzeichen raus und macht eins draus
    intermediate
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
}

pub async fn upload_receipt(
    State(state): State<Arc<AppState>>,
    body_bytes: Bytes
) -> Result<impl IntoResponse, (StatusCode, String)> {

    // 1. Hole das originale Struct aus der Pipeline
    let receipt_struct = match ocr::run_ocr_pipeline(&body_bytes, &state.config).await {
        Ok(r) => r,
        Err(err) => {
            error!("OCR Pipeline failed: {:?}", err);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("OCR Pipeline error: {}", err)));
        }
    };

    // 2. Konvertiere das Struct explizit in ein flexibles serde_json::Value
    let mut receipt_val: serde_json::Value = serde_json::to_value(receipt_struct)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to serialize OCR response: {}", e)))?;

    // 3. Typensicheres und extrem tolerantes Auslesen des Retailers aus dem Value-Baum
    let extracted_retailer = receipt_val
        .get("retailer")
        .and_then(|r| {
            if let Some(v) = r.get("value").and_then(|v| v.as_str()) {
                Some(v.to_string())
            } else if let Some(a) = r.get("address").and_then(|a| a.as_str()) {
                Some(a.to_string())
            } else if let Some(s) = r.as_str() {
                Some(s.to_string())
            } else {
                None
            }
        })
        .unwrap_or_default()
        .trim()
        .to_string();

    if !extracted_retailer.is_empty() {
        if let Some(articles_array) = receipt_val.get_mut("articles").and_then(|a| a.as_array_mut()) {
            for article in articles_array {
                if let Some(raw_name) = article.get("name").and_then(|n| n.as_str()) {

                    let clean_name = clean_ocr_name(raw_name);
                    if clean_name.is_empty() { continue; }

                    // NEU: Wir injizieren den gesäuberten OCR-Namen direkt in das JSON für das Frontend
                    article["raw_ocr_name"] = serde_json::json!(clean_name);

                    let mut matched_data = None;

                    let variant_match = sqlx::query(
                        "SELECT ep.name, ep.category, ep.brand, ep.ean \
                            FROM ean_ocr_variants v \
                            JOIN ean_products ep ON v.ean = ep.ean \
                            WHERE REPLACE(v.retailer, ' ', '') = REPLACE(?, ' ', '') COLLATE NOCASE \
                            AND REPLACE(v.raw_ocr_name, ' ', '') = REPLACE(?, ' ', '') COLLATE NOCASE \
                            LIMIT 1"
                    )
                        .bind(&extracted_retailer)
                        .bind(&clean_name)
                        .fetch_optional(&state.ean_db)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

                    if let Some(row) = variant_match {
                        let db_category: ProductCategory = row.get("category");
                        matched_data = Some((
                            row.get::<String, _>("name"),
                            db_category,
                            row.get::<String, _>("brand"),
                            Some(row.get::<String, _>("ean"))
                        ));
                    } else {
                        // 2. Prio: Historischer Match
                        let history_match = sqlx::query(
                            "SELECT p.name, p.category, p.brand, p.ean \
                                FROM products p \
                                JOIN receipt_items ri ON ri.product_id = p.id \
                                JOIN receipts r ON ri.receipt_id = r.id \
                                WHERE TRIM(p.name) = ? COLLATE NOCASE \
                                AND TRIM(r.retailer) = ? COLLATE NOCASE \
                                AND p.is_deleted = 0 \
                                LIMIT 1"
                        )
                            .bind(&clean_name)
                            .bind(&extracted_retailer)
                            .fetch_optional(&state.user_db)
                            .await
                            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

                        if let Some(row) = history_match {
                            let db_category: ProductCategory = row.get("category");
                            matched_data = Some((
                                row.get::<String, _>("name"),
                                db_category,
                                row.get::<String, _>("brand"),
                                row.get::<Option<String>, _>("ean")
                            ));
                        } else {
                            // 3. Prio: EAN-Stammdaten Match
                            let ean_match = sqlx::query(
                                "SELECT name, category, brand, ean FROM ean_products \
                                    WHERE TRIM(name) = ? COLLATE NOCASE \
                                    LIMIT 1"
                            )
                                .bind(&clean_name)
                                .fetch_optional(&state.ean_db)
                                .await
                                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

                            if let Some(row) = ean_match {
                                let db_category: ProductCategory = row.get("category");
                                matched_data = Some((
                                    row.get::<String, _>("name"),
                                    db_category,
                                    row.get::<String, _>("brand"),
                                    Some(row.get::<String, _>("ean"))
                                ));
                            }
                        }
                    }

                    if let Some((db_name, db_category, db_brand, db_ean)) = matched_data {
                        article["name"] = serde_json::json!(db_name);
                        article["category"] = serde_json::json!(db_category);

                        if db_brand != "UNKNOWN" && !db_brand.trim().is_empty() {
                            article["brand"] = serde_json::json!(db_brand);
                        }
                        if article.get("ean").and_then(|e| e.as_str()).is_none() && db_ean.is_some() {
                            article["ean"] = serde_json::json!(sanitize_ean(&db_ean.unwrap()));
                        }
                    }
                }
            }
        }
    }

    Ok(Json(receipt_val))
}

pub async fn lookup_ean(
    State(state): State<Arc<AppState>>,
    Path(ean): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let clean_ean = sanitize_ean(&ean);

    let result = sqlx::query("SELECT name, category, brand FROM ean_products WHERE ean = ?")
        .bind(&clean_ean)
        .fetch_optional(&state.ean_db)
        .await
        .map_err(|e| {
            error!("Database error during EAN lookup for {}: {}", clean_ean, e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal databases error".to_string())
        })?;

    match result {
        Some(row) => {
            let product_name: String = row.get("name");
            let category: String = row.try_get("category").unwrap_or_else(|_| "UNCATEGORIZED".to_string());
            let brand: String = row.try_get("brand").unwrap_or_else(|_| "UNKNOWN".to_string());

            let response = serde_json::json!({
                "ean": clean_ean,
                "name": product_name,
                "category": category,
                "brand": if brand.trim().is_empty() { "UNKNOWN".to_string() } else { brand },
                "found": true
            });
            Ok(Json(response))
        }
        None => {
            let response = serde_json::json!({
                "ean": clean_ean,
                "name": "",
                "category": "UNCATEGORIZED",
                "brand": "UNKNOWN",
                "found": false
            });
            Ok(Json(response))
        }
    }
}

pub async fn finalize_receipt(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<FinalizeReceiptPayload>,
) -> Result<StatusCode, (StatusCode, String)> {

    let user_db = &state.user_db;
    let mut tx = user_db.begin().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let receipt_row = sqlx::query(
        "INSERT INTO receipts (retailer, receipt_date) VALUES (?, ?) RETURNING id"
    )
        .bind(&payload.retailer)
        .bind(&payload.receipt_date)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let receipt_id: i64 = receipt_row.get("id");

    for article in payload.articles {
        let mut product_id: Option<i64> = None;
        let mut existing_ean: Option<String> = None;
        let mut existing_category: ProductCategory = ProductCategory::Uncategorized;
        let mut existing_brand: String = "UNKNOWN".to_string();

        let clean_article_ean = article.ean.as_ref().map(|e| sanitize_ean(e));

        if let Some(ref ean_code) = clean_article_ean {
            let row = sqlx::query(
                "SELECT id, ean, category, brand FROM products WHERE ean = ? AND is_deleted = 0 LIMIT 1"
            )
                .bind(ean_code)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            if let Some(r) = row {
                product_id = Some(r.get("id"));
                existing_ean = r.get::<Option<String>, _>("ean").map(|e| sanitize_ean(&e));
                existing_category = r.get("category");
                existing_brand = r.get("brand");
            }
        }

        if product_id.is_none() {
            let row = sqlx::query(
                "SELECT id, ean, category, brand FROM products WHERE name = ? AND is_deleted = 0 LIMIT 1"
            )
                .bind(&article.name)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            if let Some(r) = row {
                product_id = Some(r.get("id"));
                existing_ean = r.get::<Option<String>, _>("ean").map(|e| sanitize_ean(&e));
                existing_category = r.get("category");
                existing_brand = r.get("brand");
            }
        }

        let final_product_id = match product_id {
            Some(id) => {
                let need_ean_update = existing_ean.is_none() && clean_article_ean.is_some();
                let need_category_update = existing_category == ProductCategory::Uncategorized && article.category != ProductCategory::Uncategorized;

                let clean_existing = existing_brand.trim().to_uppercase().replace('\'', "").replace('"', "");
                let clean_incoming = article.brand.trim().to_uppercase().replace('\'', "").replace('"', "");

                let is_existing_unknown = clean_existing == "UNKNOWN" || clean_existing.is_empty();
                let is_incoming_valid = clean_incoming != "UNKNOWN" && !clean_incoming.is_empty();

                let need_brand_update = is_existing_unknown && is_incoming_valid;

                if need_ean_update || need_category_update || need_brand_update {
                    let target_ean = clean_article_ean.as_ref().or(existing_ean.as_ref());
                    let target_category = if need_category_update { article.category } else { existing_category };
                    let target_brand = if need_brand_update { &article.brand } else { &existing_brand };

                    sqlx::query("UPDATE products SET ean = ?, category = ?, brand = ? WHERE id = ?")
                        .bind(target_ean)
                        .bind(target_category)
                        .bind(target_brand)
                        .bind(id)
                        .execute(&mut *tx)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                }
                id
            },
            None => {
                let new_product_row = sqlx::query(
                    "INSERT INTO products (name, ean, category, brand) VALUES (?, ?, ?, ?) RETURNING id"
                )
                    .bind(&article.name)
                    .bind(&clean_article_ean)
                    .bind(article.category)
                    .bind(&article.brand)
                    .fetch_one(&mut *tx)
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

                new_product_row.get("id")
            }
        };

        let price_in_cents = (article.price * 100.0).round() as i64;

        sqlx::query(
            "INSERT INTO receipt_items (receipt_id, product_id, price, quantity) VALUES (?, ?, ?, ?)"
        )
            .bind(receipt_id)
            .bind(final_product_id)
            .bind(price_in_cents)
            .bind(article.quantity)
            .execute(&mut *tx)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        if let Some(ref ean_code) = clean_article_ean {
            if !ean_code.trim().is_empty() {
                if let Err(e) = sqlx::query(
                    "INSERT OR IGNORE INTO ean_products (ean, name, category, brand) VALUES (?, ?, ?, ?)"
                )
                    .bind(ean_code)
                    .bind(&article.name)
                    .bind(article.category.clone())
                    .bind(&article.brand)
                    .execute(&state.ean_db)
                .await
                {
                    error!("Konnte ean_products nicht aktualisieren: {}", e);
                }

                // NEU: Nur ins Wörterbuch eintragen, wenn wir einen gültigen raw_ocr_name haben
                let cleaned_ocr_field = clean_ocr_name(&article.raw_ocr_name);
                if !cleaned_ocr_field.is_empty() {
                    if let Err(e) = sqlx::query(
                        "INSERT INTO ean_ocr_variants (ean, retailer, raw_ocr_name, confirmed_count) \
                            VALUES (?, ?, ?, 1) \
                            ON CONFLICT(ean, retailer, raw_ocr_name) \
                            DO UPDATE SET confirmed_count = confirmed_count + 1, updated_at = CURRENT_TIMESTAMP"
                    )
                        .bind(ean_code)
                        .bind(&payload.retailer)
                        .bind(&cleaned_ocr_field) // <-- Hier nutzen wir den sauberen OCR-Namen vom Bon!
                        .execute(&state.ean_db)
                    .await
                    {
                        error!("Konnte ean_ocr_variants Wörterbuch-Eintrag nicht updaten/einfügen: {}", e);
                    }
                }
            }
        }
    }

    tx.commit().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::CREATED)
}

pub async fn get_pantry_summary(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let rows = sqlx::query(
        "SELECT p.category, SUM(ri.quantity) as total_quantity, CAST(SUM(ri.price * ri.quantity) AS INTEGER) as total_price_cents \
            FROM receipt_items ri JOIN products p ON ri.product_id = p.id WHERE p.is_deleted = 0 GROUP BY p.category"
    )
        .fetch_all(&state.user_db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut summary = serde_json::json!([]);
    for row in rows {
        let category: String = row.get("category");
        let total_quantity: f64 = row.get("total_quantity");
        let total_price_cents: i64 = row.get("total_price_cents");

        summary.as_array_mut().unwrap().push(serde_json::json!({
            "category": category,
            "total_quantity": total_quantity,
            "total_price": (total_price_cents as f64) / 100.0
        }));
    }
    Ok(Json(summary))
}

pub async fn get_pantry_items(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let rows = sqlx::query(
        "SELECT p.name, p.brand, p.category, SUM(ri.quantity) as quantity, CAST(ri.price AS REAL) / 100.0 as unit_price \
            FROM receipt_items ri JOIN products p ON ri.product_id = p.id WHERE p.is_deleted = 0 GROUP BY p.name, p.brand, p.category, ri.price"
    )
        .fetch_all(&state.user_db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut items = serde_json::json!([]);
    for row in rows {
        let name: String = row.get("name");
        let brand: String = row.get("brand");
        let category: String = row.get("category");
        let quantity: f64 = row.get("quantity");
        let unit_price: f64 = row.get("unit_price");

        items.as_array_mut().unwrap().push(serde_json::json!({
            "name": name,
            "brand": brand,
            "category": category,
            "quantity": quantity,
            "price": unit_price * quantity
        }));
    }
    Ok(Json(items))
}

pub async fn revalidate_articles(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RevalidateReceiptPayload>,
) -> Result<impl IntoResponse, (StatusCode, String)> {

    let mut articles = payload.articles;
    let retailer = payload.retailer.trim();

    if !retailer.is_empty() {
        for article in &mut articles {
            // NEU: Wenn ein raw_ocr_name mitgeschickt wurde, suchen wir mit diesem (da article.name editiert sein könnte).
            // Falls leer (z.B. neu hinzugefügter Artikel im Frontend), weichen wir auf article.name aus.
            let source_name = if !article.raw_ocr_name.is_empty() {
                &article.raw_ocr_name
            } else {
                &article.name
            };

            let clean_name = clean_ocr_name(source_name);
            if clean_name.is_empty() { continue; }

            let mut matched_data = None;

            let variant_match = sqlx::query(
                "SELECT ep.name, ep.category, ep.brand, ep.ean \
                    FROM ean_ocr_variants v \
                    JOIN ean_products ep ON v.ean = ep.ean \
                    WHERE REPLACE(v.retailer, ' ', '') = REPLACE(?, ' ', '') COLLATE NOCASE \
                    AND REPLACE(v.raw_ocr_name, ' ', '') = REPLACE(?, ' ', '') COLLATE NOCASE \
                    LIMIT 1"
            )
                .bind(retailer)
                .bind(&clean_name)
                .fetch_optional(&state.ean_db)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            if let Some(row) = variant_match {
                let db_category: ProductCategory = row.get("category");
                matched_data = Some((
                    row.get::<String, _>("name"),
                    db_category,
                    row.get::<String, _>("brand"),
                    Some(row.get::<String, _>("ean"))
                ));
            } else {
                // 2. Prio: Historischer Match
                let history_match = sqlx::query(
                    "SELECT p.name, p.category, p.brand, p.ean \
                        FROM products p \
                        JOIN receipt_items ri ON ri.product_id = p.id \
                        JOIN receipts r ON ri.receipt_id = r.id \
                        WHERE TRIM(p.name) = ? COLLATE NOCASE \
                        AND TRIM(r.retailer) = ? COLLATE NOCASE \
                        AND p.is_deleted = 0 \
                        LIMIT 1"
                )
                    .bind(&clean_name)
                    .bind(retailer)
                    .fetch_optional(&state.user_db)
                    .await
                    .map_err(|e| {
                        error!("Database error during historical revalidation: {}", e);
                        (StatusCode::INTERNAL_SERVER_ERROR, "Internal database error".to_string())
                    })?;

                if let Some(row) = history_match {
                    let db_category: ProductCategory = row.get("category");
                    matched_data = Some((
                        row.get::<String, _>("name"),
                        db_category,
                        row.get::<String, _>("brand"),
                        row.get::<Option<String>, _>("ean")
                    ));
                } else {
                    // 3. Prio: Globale EAN-Stammdaten
                    let ean_match = sqlx::query(
                        "SELECT name, category, brand, ean FROM ean_products \
                            WHERE TRIM(name) = ? COLLATE NOCASE \
                            LIMIT 1"
                    )
                        .bind(&clean_name)
                        .fetch_optional(&state.ean_db)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

                    if let Some(row) = ean_match {
                        let db_category: ProductCategory = row.get("category");
                        matched_data = Some((
                            row.get::<String, _>("name"),
                            db_category,
                            row.get::<String, _>("brand"),
                            Some(row.get::<String, _>("ean"))
                        ));
                    }
                }
            }

            if let Some((db_name, db_category, db_brand, db_ean)) = matched_data {
                article.name = db_name;
                article.category = db_category;

                if db_brand != "UNKNOWN" && !db_brand.trim().is_empty() {
                    article.brand = db_brand;
                }
                if article.ean.is_none() {
                    article.ean = db_ean;
                }
            }
        }
    }

    Ok(Json(articles))
}
