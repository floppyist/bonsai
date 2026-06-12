use core::fmt;

#[derive(Debug)]
pub enum DBError {
    Creation,
    Migration,
    Transaction,
    Query,
}

impl fmt::Display for DBError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DBError::Creation => write!(f, "Failed to create database or pool"),
            DBError::Migration => write!(f, "Database migration failed"),
            DBError::Query => write!(f, "Database query execution failed"),
            DBError::Transaction => write!(f, "Database transaction error"),
        }
    }
}

impl std::error::Error for DBError {}
