mod error;

use log::{debug, error, info};
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Row, Sqlite, SqlitePool, migrate};

use crate::api::categories::ProductCategory;
use crate::api::models::FinalizeReceiptPayload;
use crate::config::BonsaiOptions;
use crate::db::error::DBError;
use crate::util;

#[derive(Clone)]
pub struct DatabasePools {
    pub user_db: SqlitePool,
    pub ean_db: SqlitePool,
}

#[derive(Debug, Clone)]
pub struct EanProduct {
    pub name: String,
    pub category: String,
    pub brand: String,
}

#[derive(Debug, Clone)]
pub struct MatchedProductData {
    pub name: String,
    pub category: ProductCategory,
    pub brand: String,
    pub ean: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PantrySummaryItem {
    pub category: String,
    pub total_quantity: f64,
    pub total_price_cents: i64,
}

#[derive(Debug, Clone)]
pub struct PantryItem {
    pub name: String,
    pub brand: String,
    pub category: String,
    pub quantity: f64,
    pub unit_price: f64,
}

#[derive(Debug, Clone)]
pub struct ReceiptSummaryRow {
    pub id: i64,
    pub retailer: String,
    pub receipt_date: Option<String>,
    pub created_at: String,
    pub item_count: i64,
    pub total_price_cents: i64,
}

#[derive(Debug, Clone)]
pub struct ReceiptDetailItem {
    pub name: String,
    pub brand: String,
    pub category: String,
    pub ean: Option<String>,
    pub quantity: f64,
    pub unit_price_cents: i64,
}

fn map_row_to_product(row: sqlx::sqlite::SqliteRow) -> MatchedProductData {
    MatchedProductData {
        name: row.get("name"),
        category: row.get("category"),
        brand: row.get("brand"),
        ean: row.try_get("ean").ok(),
    }
}

pub async fn establish_database_connections(
    options: &BonsaiOptions,
) -> Result<DatabasePools, DBError> {
    // User DB
    let user_db_url = &options.database.user_db_url;

    if !Sqlite::database_exists(user_db_url).await.unwrap_or(false) {
        info!("Creating database {}", user_db_url);

        if let Err(e) = Sqlite::create_database(user_db_url).await {
            error!("Database creation failed: '{}'", e);

            return Err(DBError::Creation);
        }
    } else {
        debug!("Database '{}' already exists", user_db_url);
    }

    let user_pool = SqlitePoolOptions::new()
        .max_connections(options.database.user_db_max_pool_connections)
        .connect(user_db_url)
        .await
        .map_err(|_| DBError::Creation)?;

    debug!("Process DB migration for {}...", user_db_url);

    if let Err(e) = migrate!("./migrations/user").run(&user_pool).await {
        error!("Migration failed: '{}'", e);

        return Err(DBError::Migration);
    }

    info!("Connected to database: '{}'", user_db_url);

    // EAN DB
    let ean_db_url = &options.database.ean_db_url;

    if !Sqlite::database_exists(ean_db_url).await.unwrap_or(false) {
        info!("Creating database {}", ean_db_url);

        if let Err(e) = Sqlite::create_database(ean_db_url).await {
            error!("Database creation failed: '{}'", e);

            return Err(DBError::Creation);
        }
    } else {
        debug!("Database '{}' already exists", ean_db_url);
    }

    let ean_pool = SqlitePoolOptions::new()
        .max_connections(options.database.ean_db_max_pool_connections)
        .connect(ean_db_url)
        .await
        .map_err(|_| DBError::Creation)?;

    debug!("Process DB migration for {}...", ean_db_url);

    if let Err(e) = migrate!("./migrations/ean").run(&ean_pool).await {
        error!("Migration failed: '{}'", e);

        return Err(DBError::Migration);
    }

    info!("Connected to database: '{}'", ean_db_url);

    Ok(DatabasePools {
        user_db: user_pool,
        ean_db: ean_pool,
    })
}

pub async fn fetch_product_by_ean(
    pool: &SqlitePool,
    ean: &str,
) -> Result<Option<EanProduct>, DBError> {
    let row = sqlx::query("SELECT name, category, brand FROM ean_products WHERE ean = ?")
        .bind(ean)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            error!("Database error during EAN lookup for {}: {}", ean, e);
            DBError::Query
        })?;

    if let Some(row) = row {
        let name: String = row.get("name");
        let category: String = row
            .try_get("category")
            .unwrap_or_else(|_| "UNCATEGORIZED".to_string());
        let brand: String = row
            .try_get("brand")
            .unwrap_or_else(|_| "UNKNOWN".to_string());

        Ok(Some(EanProduct {
            name,
            category,
            brand,
        }))
    } else {
        Ok(None)
    }
}

pub async fn resolve_article_metadata(
    ean_db: &SqlitePool,
    user_db: &SqlitePool,
    retailer: &str,
    clean_name: &str,
    user_id: i32,
) -> Result<Option<MatchedProductData>, DBError> {
    // OCR variants check
    if let Some(row) = sqlx::query(
        "SELECT ep.name, ep.category, ep.brand, ep.ean FROM ean_ocr_variants v \
            JOIN ean_products ep ON v.ean = ep.ean \
            WHERE REPLACE(v.retailer, ' ', '') = REPLACE(?, ' ', '') COLLATE NOCASE \
            AND REPLACE(v.raw_ocr_name, ' ', '') = REPLACE(?, ' ', '') COLLATE NOCASE LIMIT 1",
    )
    .bind(retailer)
    .bind(clean_name)
    .fetch_optional(ean_db)
    .await
    .map_err(|_| DBError::Query)?
    {
        return Ok(Some(map_row_to_product(row)));
    }

    // User history
    if let Some(row) = sqlx::query(
        "SELECT p.name, p.category, p.brand, p.ean FROM products p \
            JOIN receipt_items ri ON ri.product_id = p.id \
            JOIN receipts r ON ri.receipt_id = r.id \
            WHERE TRIM(p.name) = ? COLLATE NOCASE AND TRIM(r.retailer) = ? COLLATE NOCASE \
            AND p.is_deleted = 0 AND r.user_id = ? LIMIT 1",
    )
    .bind(clean_name)
    .bind(retailer)
    .bind(user_id)
    .fetch_optional(user_db)
    .await
    .map_err(|_| DBError::Query)?
    {
        return Ok(Some(map_row_to_product(row)));
    }

    // Global EAN products
    if let Some(row) = sqlx::query(
        "SELECT name, category, brand, ean FROM ean_products \
            WHERE TRIM(name) = ? COLLATE NOCASE LIMIT 1",
    )
    .bind(clean_name)
    .fetch_optional(ean_db)
    .await
    .map_err(|_| DBError::Query)?
    {
        return Ok(Some(map_row_to_product(row)));
    }

    Ok(None)
}

// Atomically saves a finalized receipt within a database transaction,
// populating both the user history and the global EAN/OCR dictionary.
pub async fn save_finalized_receipt(
    user_db: &SqlitePool,
    ean_db: &SqlitePool,
    payload: FinalizeReceiptPayload,
    user_id: i32,
) -> Result<(), DBError> {
    let mut tx = user_db.begin().await.map_err(|_| DBError::Transaction)?;

    // Insert the main receipt record
    let receipt_row =
        sqlx::query("INSERT INTO receipts (user_id, retailer, receipt_date) VALUES (?, ?, ?) RETURNING id")
            .bind(user_id)
            .bind(&payload.retailer)
            .bind(&payload.receipt_date)
            .fetch_one(&mut *tx)
            .await
            .map_err(|_| DBError::Query)?;

    let receipt_id: i64 = receipt_row.get("id");

    // Process each individual article
    for article in payload.articles {
        let mut product_id: Option<i64> = None;
        let mut existing_ean: Option<String> = None;
        let mut existing_category = ProductCategory::Uncategorized;
        let mut existing_brand = "UNKNOWN".to_string();

        let clean_article_ean = article.ean.as_ref().map(|e| util::sanitize_ean(e));

        // Attempt matching via EAN code
        if let Some(ref ean_code) = clean_article_ean {
            let row = sqlx::query(
                "SELECT id, ean, category, brand FROM products WHERE ean = ? AND is_deleted = 0 AND user_id = ? LIMIT 1"
            )
                .bind(ean_code)
                .bind(user_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|_| DBError::Query)?;

            if let Some(r) = row {
                product_id = Some(r.get("id"));
                existing_ean = r
                    .get::<Option<String>, _>("ean")
                    .map(|e| util::sanitize_ean(&e));
                existing_category = r.get("category");
                existing_brand = r.get("brand");
            }
        }

        // Fallback: Attempt matching via product name
        if product_id.is_none() {
            let row = sqlx::query(
                "SELECT id, ean, category, brand FROM products WHERE name = ? AND is_deleted = 0 AND user_id = ? LIMIT 1"
            )
                .bind(&article.name)
                .bind(user_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|_| DBError::Query)?;

            if let Some(r) = row {
                product_id = Some(r.get("id"));
                existing_ean = r
                    .get::<Option<String>, _>("ean")
                    .map(|e| util::sanitize_ean(&e));
                existing_category = r.get("category");
                existing_brand = r.get("brand");
            }
        }

        // Create or update the product details
        let final_product_id = match product_id {
            Some(id) => {
                let need_ean_update = existing_ean.is_none() && clean_article_ean.is_some();
                let need_category_update = existing_category == ProductCategory::Uncategorized
                && article.category != ProductCategory::Uncategorized;

                let clean_existing = existing_brand.trim().to_uppercase().replace(['\'', '"'], "");
                let clean_incoming = article.brand.trim().to_uppercase().replace(['\'', '"'], "");
                let need_brand_update = (clean_existing == "UNKNOWN" || clean_existing.is_empty()) 
                && (clean_incoming != "UNKNOWN" && !clean_incoming.is_empty());

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
                        .map_err(|_| DBError::Query)?;
                }

                id
            }
            None => {
                sqlx::query("INSERT INTO products (user_id, name, ean, category, brand) VALUES (?, ?, ?, ?, ?) RETURNING id")
                    .bind(user_id)
                    .bind(&article.name)
                    .bind(&clean_article_ean)
                    .bind(article.category)
                    .bind(&article.brand)
                    .fetch_one(&mut *tx)
                    .await
                    .map_err(|_| DBError::Query)?
                    .get("id")
            }
        };

        // Link the item to the receipt
        let price_in_cents = (article.price * 100.0).round() as i64;

        sqlx::query("INSERT INTO receipt_items (receipt_id, product_id, price, quantity) VALUES (?, ?, ?, ?)")
            .bind(receipt_id)
            .bind(final_product_id)
            .bind(price_in_cents)
            .bind(article.quantity)
            .execute(&mut *tx)
            .await
            .map_err(|_| DBError::Query)?;

        // Enrich the global EAN data dictionary asynchronously (ignoring failures gracefully)
        if let Some(ref ean_code) = clean_article_ean
            && !ean_code.trim().is_empty()
        {
            let _ = sqlx::query("INSERT OR IGNORE INTO ean_products (ean, name, category, brand) VALUES (?, ?, ?, ?)")
                .bind(ean_code)
                .bind(&article.name)
                .bind(article.category)
                .bind(&article.brand)
                .execute(ean_db)
            .await;

            let cleaned_ocr_field = util::clean_ocr_name(&article.raw_ocr_name);
            if !cleaned_ocr_field.is_empty() {
                let _ = sqlx::query(
                    "INSERT INTO ean_ocr_variants (ean, retailer, raw_ocr_name, confirmed_count) \
                        VALUES (?, ?, ?, 1) \
                        ON CONFLICT(ean, retailer, raw_ocr_name) \
                        DO UPDATE SET confirmed_count = confirmed_count + 1, updated_at = CURRENT_TIMESTAMP"
                )
                    .bind(ean_code)
                    .bind(&payload.retailer)
                    .bind(&cleaned_ocr_field)
                    .execute(ean_db)
                .await;
            }
        }
    }

    tx.commit().await.map_err(|_| DBError::Transaction)
}

// Fetches the aggregated category breakdown metrics from the user database
pub async fn fetch_pantry_summary(user_db: &SqlitePool, user_id: i32) -> Result<Vec<PantrySummaryItem>, DBError> {
    sqlx::query(
        "SELECT p.category, SUM(ri.quantity) as total_quantity, CAST(SUM(ri.price * ri.quantity) AS INTEGER) as total_price_cents \
            FROM receipt_items ri JOIN products p ON ri.product_id = p.id WHERE p.is_deleted = 0 AND p.user_id = ? GROUP BY p.category"
    )
        .bind(user_id)
        .fetch_all(user_db)
        .await
        .map_err(|_| DBError::Query)?
        .into_iter()
        .map(|row| Ok(PantrySummaryItem {
            category: row.get("category"),
            total_quantity: row.get("total_quantity"),
            total_price_cents: row.get("total_price_cents"),
        }))
        .collect()
}

// Fetches the receipt history list with aggregated item counts and totals
pub async fn fetch_receipts_history(
    user_db: &SqlitePool,
    user_id: i32,
) -> Result<Vec<ReceiptSummaryRow>, DBError> {
    sqlx::query(
        "SELECT r.id, r.retailer, r.receipt_date, r.created_at, \
            COUNT(ri.id) as item_count, \
            CAST(COALESCE(SUM(ri.price * ri.quantity), 0) AS INTEGER) as total_price_cents \
            FROM receipts r LEFT JOIN receipt_items ri ON ri.receipt_id = r.id \
            WHERE r.user_id = ? \
            GROUP BY r.id ORDER BY COALESCE(r.receipt_date, r.created_at) DESC, r.id DESC",
    )
    .bind(user_id)
    .fetch_all(user_db)
    .await
    .map_err(|_| DBError::Query)?
    .into_iter()
    .map(|row| {
        Ok(ReceiptSummaryRow {
            id: row.get("id"),
            retailer: row.get("retailer"),
            receipt_date: row.try_get("receipt_date").ok(),
            created_at: row.get("created_at"),
            item_count: row.get("item_count"),
            total_price_cents: row.get("total_price_cents"),
        })
    })
    .collect()
}

// Fetches all individual articles captured during the scan of a specific receipt
pub async fn fetch_receipt_details(
    user_db: &SqlitePool,
    receipt_id: i64,
    user_id: i32,
) -> Result<Vec<ReceiptDetailItem>, DBError> {
    sqlx::query(
        "SELECT p.name, p.brand, p.category, p.ean, ri.quantity, ri.price as unit_price_cents \
            FROM receipt_items ri \
            JOIN products p ON ri.product_id = p.id \
            JOIN receipts r ON ri.receipt_id = r.id \
            WHERE ri.receipt_id = ? AND r.user_id = ? ORDER BY ri.id ASC",
    )
    .bind(receipt_id)
    .bind(user_id)
    .fetch_all(user_db)
    .await
    .map_err(|e| {
        error!("Error fetching receipt details: {}", e);
        DBError::Query
    })?
    .into_iter()
    .map(|row| {
        Ok(ReceiptDetailItem {
            name: row.get("name"),
            brand: row.get("brand"),
            category: row.get("category"),
            ean: row.try_get("ean").ok(),
            quantity: row.get("quantity"),
            unit_price_cents: row.get("unit_price_cents"),
        })
    })
    .collect()
}

// Fetches all individual tracking entities currently stocked inside the user pantry
pub async fn fetch_pantry_items(user_db: &SqlitePool, user_id: i32) -> Result<Vec<PantryItem>, DBError> {
    sqlx::query(
        "SELECT p.name, p.brand, p.category, SUM(ri.quantity) as quantity, CAST(ri.price AS REAL) / 100.0 as unit_price \
            FROM receipt_items ri JOIN products p ON ri.product_id = p.id WHERE p.is_deleted = 0 AND p.user_id = ? GROUP BY p.name, p.brand, p.category, ri.price"
    )
        .bind(user_id)
        .fetch_all(user_db)
        .await
        .map_err(|_| DBError::Query)?
        .into_iter()
        .map(|row| Ok(PantryItem {
            name: row.get("name"),
            brand: row.get("brand"),
            category: row.get("category"),
            quantity: row.get("quantity"),
            unit_price: row.get("unit_price"),
        }))
        .collect()
}

pub async fn create_user(pool: &SqlitePool, username: &str, password_hash: &str) -> Result<i64, DBError> {
    let row = sqlx::query("INSERT INTO users (username, password_hash) VALUES (?, ?) RETURNING id")
        .bind(username)
        .bind(password_hash)
        .fetch_one(pool)
        .await
        .map_err(|_| DBError::Query)?;
    
    Ok(row.get("id"))
}

pub async fn get_user_by_username(pool: &SqlitePool, username: &str) -> Result<Option<(i32, String)>, DBError> {
    let row = sqlx::query("SELECT id, password_hash FROM users WHERE username = ?")
        .bind(username)
        .fetch_optional(pool)
        .await
        .map_err(|_| DBError::Query)?;
    
    Ok(row.map(|r| (r.get("id"), r.get("password_hash"))))
}

pub async fn get_username_by_id(pool: &SqlitePool, user_id: i32) -> Result<Option<String>, DBError> {
    let row = sqlx::query("SELECT username FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_optional(pool)
        .await
        .map_err(|_| DBError::Query)?;

    Ok(row.map(|r| r.get("username")))
}
