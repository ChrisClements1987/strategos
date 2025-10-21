use rusqlite::{Connection, Result};
use std::path::PathBuf;

pub fn init_db(db_path: &PathBuf) -> Result<Connection> {
    let conn = Connection::open(db_path)?;
    
    // Execute schema
    let schema = include_str!("schema.sql");
    conn.execute_batch(schema)?;
    
    Ok(conn)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_init_db_creates_tables() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_path_buf();
        
        let conn = init_db(&db_path).unwrap();
        
        // Verify portfolios table exists
        let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='portfolios'").unwrap();
        let table_exists = stmt.exists([]).unwrap();
        
        assert!(table_exists, "portfolios table should exist");
    }
}
