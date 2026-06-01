mod error;
mod status;

use log::debug;
use log::info;
use log::error;

use sqlx::Sqlite;
use sqlx::migrate;
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::SqlitePoolOptions;

use crate::config::BonsaiOptions;
use crate::db::error::DBError;
use crate::db::status::DBStatus;

pub async fn establish_database_connection(options: &BonsaiOptions) -> Result<DBStatus, DBError> {
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
        .max_connections(5)
        .connect(db_url)
        .await.unwrap();

    debug!("Process DB migration...");

    if let Err(e) = migrate!("./migrations").run(&pool).await {
        error!("Migration failed: '{}'", e);
        return Err(DBError::MigrationFailed);
    }

    info!("Connected to database: '{}'", db_url);

    Ok(DBStatus::DatabaseConnected)
}
