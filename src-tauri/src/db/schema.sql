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
