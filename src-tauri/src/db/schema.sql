-- Strategos Database Schema
-- Version: 0.1.0

-- Portfolio table
CREATE TABLE IF NOT EXISTS portfolios (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Trigger to update updated_at on portfolio changes
CREATE TRIGGER IF NOT EXISTS update_portfolio_timestamp
AFTER UPDATE ON portfolios
FOR EACH ROW
BEGIN
    UPDATE portfolios SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
END;

-- Product table (temporary for feature tests)
CREATE TABLE IF NOT EXISTS products (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    portfolio_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    product_type TEXT NOT NULL DEFAULT 'product',
    status TEXT NOT NULL DEFAULT 'active',
    FOREIGN KEY (portfolio_id) REFERENCES portfolios(id) ON DELETE CASCADE
);

-- Feature table
CREATE TABLE IF NOT EXISTS features (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    product_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'planned',
    priority TEXT NOT NULL DEFAULT 'medium',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE
);

-- Trigger to update updated_at on feature changes
CREATE TRIGGER IF NOT EXISTS update_feature_timestamp
AFTER UPDATE ON features
FOR EACH ROW
BEGIN
    UPDATE features SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
END;
