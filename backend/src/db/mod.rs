mod error;

use log::debug;
use log::info;
use log::error;

use sqlx::Sqlite;
use sqlx::SqlitePool;
use sqlx::migrate;
use sqlx::migrate::MigrateDatabase;
use sqlx::query;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::Row;

use uuid::Uuid;

use crate::config::BonsaiOptions;
use crate::db::error::DBError;
use crate::ocr::Receipt;

pub async fn establish_database_connection(options: &BonsaiOptions) -> Result<SqlitePool, DBError> {
    let db_url = &options.database.url;

    if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
        info!("Creating database {}", db_url);

        match Sqlite::create_database(db_url).await {
            Ok(_) => {
                info!("{} created successfully", db_url);
            },
            Err(e) => {
                error!("Database creation failed: '{}'", e);

                return Err(DBError::CreationFailed);
            }
        }
    } else {
        debug!("Database '{}' already exists", db_url);
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(options.database.max_pool_connections)
        .connect(db_url)
        .await.unwrap();

    debug!("Process DB migration...");

    if let Err(e) = migrate!("./migrations").run(&pool).await {
        error!("Migration failed: '{}'", e);
        return Err(DBError::MigrationFailed);
    }

    info!("Connected to database: '{}'", db_url);

    Ok(pool)
}

pub async fn save_receipt(pool: &SqlitePool, receipt: &Receipt) -> Result<(), DBError> {
    let mut tx = pool.begin().await.map_err(|e| {
        error!("Failed to start transaction: {}", e);

        DBError::TransactionFailed
    })?;

    let receipt_id = Uuid::new_v4().to_string();

    query("INSERT INTO receipts (id, retailer, date, total_amount) VALUES (?, ?, ?, ?)")
        .bind(&receipt_id)
        .bind(&receipt.retailer)
        .bind(&receipt.date)
        .bind(receipt.total_amount)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            error!("Failed to insert receipt: {}", e);
            DBError::QueryFailed
        })?;

    for item in &receipt.articles {
        let product_id: i64 = query(
            "INSERT INTO products (name) VALUES (?) ON CONFLICT(name) DO UPDATE SET name=name RETURNING id"
        )
            .bind(&item.name)
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| {
                error!("Failed to upsert product '{}': {}", item.name, e);
                DBError::QueryFailed
            })?
            .get("id");

        sqlx::query(
            "INSERT INTO receipt_items (receipt_id, product_id, quantity, price_unit, price_total) VALUES (?, ?, ?, ?, ?)"
        )
            .bind(&receipt_id)
            .bind(product_id)
            .bind(item.quantity)
            .bind(item.price_unit)
            .bind(item.price_total)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                error!("Failed to insert receipt item: {}", e);

                DBError::QueryFailed
            })?;
    }

    tx.commit().await.map_err(|e| {
        error!("Failed to commit transaction: {}", e);

        DBError::TransactionFailed
    })?;

    Ok(())
}
