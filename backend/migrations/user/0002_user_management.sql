CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE COLLATE NOCASE,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

PRAGMA foreign_keys=OFF;

CREATE TABLE new_receipts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    retailer TEXT NOT NULL,
    receipt_date TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE new_products (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    ean TEXT DEFAULT NULL,
    category TEXT NOT NULL DEFAULT 'UNCATEGORIZED',
    brand TEXT NOT NULL DEFAULT 'UNKNOWN',
    is_deleted INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE new_receipt_items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    receipt_id INTEGER NOT NULL,
    product_id INTEGER NOT NULL,
    price INTEGER NOT NULL,
    quantity REAL NOT NULL DEFAULT 1.0,
    FOREIGN KEY (receipt_id) REFERENCES new_receipts(id) ON DELETE CASCADE,
    FOREIGN KEY (product_id) REFERENCES new_products(id) ON DELETE RESTRICT
);

DROP TABLE receipt_items;
DROP TABLE receipts;
DROP TABLE products;

ALTER TABLE new_receipts RENAME TO receipts;
ALTER TABLE new_products RENAME TO products;
ALTER TABLE new_receipt_items RENAME TO receipt_items;

CREATE INDEX IF NOT EXISTS idx_receipts_user_date ON receipts (user_id, receipt_date);
CREATE INDEX IF NOT EXISTS idx_products_user_sync ON products (user_id, updated_at);
CREATE INDEX IF NOT EXISTS idx_receipt_items_receipt ON receipt_items (receipt_id);

PRAGMA foreign_keys=ON;
