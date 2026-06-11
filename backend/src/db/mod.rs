mod error;

use log::{debug, error, info};
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Row, Sqlite, SqlitePool, migrate};

use crate::api::categories::ProductCategory;
use crate::config::BonsaiOptions;
use crate::db::error::DBError;

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
    /* User DB */
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

    /* EAN DB */
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
) -> Result<Option<MatchedProductData>, DBError> {
    /* OCR variants check */
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

    /* User history */
    if let Some(row) = sqlx::query(
        "SELECT p.name, p.category, p.brand, p.ean FROM products p \
            JOIN receipt_items ri ON ri.product_id = p.id \
            JOIN receipts r ON ri.receipt_id = r.id \
            WHERE TRIM(p.name) = ? COLLATE NOCASE AND TRIM(r.retailer) = ? COLLATE NOCASE \
            AND p.is_deleted = 0 LIMIT 1",
    )
    .bind(clean_name)
    .bind(retailer)
    .fetch_optional(user_db)
    .await
    .map_err(|_| DBError::Query)?
    {
        return Ok(Some(map_row_to_product(row)));
    }

    /* Global EAN products */
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
