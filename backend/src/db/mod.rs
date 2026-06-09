mod error;

use log::{debug, error, info};
use sqlx::{migrate, Row, Sqlite, SqlitePool};
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::SqlitePoolOptions;

use crate::config::BonsaiOptions;
use crate::db::error::DBError;
use crate::ocr::Receipt;

#[derive(Clone)]
pub struct DatabasePools {
    pub user_db: SqlitePool,
    pub ean_db: SqlitePool,
}

pub async fn establish_database_connections(options: &BonsaiOptions) -> Result<DatabasePools, DBError> {
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

pub async fn save_receipt(pool: &SqlitePool, receipt: &Receipt) -> Result<(), DBError> {
    let mut tx = pool.begin().await.map_err(|e| {
        error!("Failed to start transaction: {}", e);
        DBError::Transaction
    })?;

    let receipt_id: i64 = sqlx::query(
        "INSERT INTO receipts (retailer, receipt_date) VALUES (?, ?) RETURNING id"
    )
    .bind(&receipt.retailer.value)
    .bind(&receipt.date.value)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| {
        error!("Failed to insert receipt: {}", e);
        DBError::Query
    })?
    .get("id");

    for item in &receipt.articles {
        let product_match = sqlx::query(
            "SELECT id FROM products WHERE name = ? AND is_deleted = 0 LIMIT 1"
        )
        .bind(&item.name)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| {
            error!("Failed to query product '{}': {}", item.name, e);
            DBError::Query
        })?;

        let product_id: i64 = match product_match {
            Some(row) => row.get("id"),
            None => {
                sqlx::query(
                    "INSERT INTO products (name, ean) VALUES (?, ?) RETURNING id"
                )
                .bind(&item.name)
                .bind(None::<&str>)
                .fetch_one(&mut *tx)
                .await
                .map_err(|e| {
                    error!("Failed to insert product '{}': {}", item.name, e);
                    DBError::Query
                })?
                .get("id")
            }
        };

        let price_in_cents = (item.price_total * 100.0).round() as i64;

        sqlx::query(
            "INSERT INTO receipt_items (receipt_id, product_id, quantity, price) VALUES (?, ?, ?, ?)"
        )
        .bind(receipt_id)
        .bind(product_id)
        .bind(item.quantity)
        .bind(price_in_cents)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            error!("Failed to insert receipt item: {}", e);
            DBError::Query
        })?;
    }

    tx.commit().await.map_err(|e| {
        error!("Failed to commit transaction: {}", e);
        DBError::Transaction
    })?;

    Ok(())
}

pub async fn get_product_by_name(pool: &SqlitePool, name: &str) -> Result<Option<i64>, DBError> {
    let result = sqlx::query("SELECT id FROM products WHERE name = ? AND is_deleted = 0 LIMIT 1")
        .bind(name)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            error!("Failed to query product by name: {}", e);
            DBError::Query
        })?;

    Ok(result.map(|row| row.get("id")))
}

pub async fn search_ean_variant(pool: &SqlitePool, retailer: &str, raw_ocr_name: &str) -> Result<Option<String>, DBError> {
    let result = sqlx::query("SELECT ean FROM ean_ocr_variants WHERE retailer = ? AND raw_ocr_name = ? LIMIT 1")
        .bind(retailer)
        .bind(raw_ocr_name)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            error!("Failed to query ean variants: {}", e);
            DBError::Query
        })?;

    Ok(result.map(|row| row.get("ean")))
}
