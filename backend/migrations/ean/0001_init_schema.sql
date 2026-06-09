-- ean.db

CREATE TABLE IF NOT EXISTS ean_products (
    ean TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    brand TEXT NOT NULL DEFAULT 'UNKNOWN',
    category TEXT NOT NULL DEFAULT 'UNCATEGORIZED',
    nutrients TEXT DEFAULT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS ean_ocr_variants (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ean TEXT NOT NULL,
    retailer TEXT NOT NULL,
    raw_ocr_name TEXT NOT NULL,
    confirmed_count INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (ean) REFERENCES ean_products(ean) ON DELETE CASCADE,
    UNIQUE(ean, retailer, raw_ocr_name)
);

CREATE INDEX IF NOT EXISTS idx_ocr_variants_search 
ON ean_ocr_variants (retailer, raw_ocr_name);
