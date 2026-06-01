use log::{error, info};

use sqlx::Sqlite;
use sqlx::migrate::MigrateDatabase;

use crate::config::BonsaiOptions;
use crate::db::error::DBError;
use crate::db::status::DBStatus;

pub mod error;
pub mod status;

pub async fn establish_database_connection(options: &BonsaiOptions) -> Result<DBStatus, DBError> {
    let db_url = &options.database.url;

    if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
        info!("Creating database {}", db_url);

        match Sqlite::create_database(db_url).await {
            Ok(_) => {
                info!("{} created successfully", db_url);

                Ok(DBStatus::DatabaseCreated)
            },
            Err(e) => {
                error!("Database creation crashed: '{}'", e);

                Err(DBError::CreationFailed)
            }
        }
    } else {
        info!("Database {} loaded", db_url);

        Ok(DBStatus::DatabaseExists)
    }
}
