use crate::models::Portfolio;
use rusqlite::{Connection, Result, params};

pub struct PortfolioService;

impl PortfolioService {
    pub fn create(conn: &Connection, portfolio: &Portfolio) -> Result<i64> {
        conn.execute(
            "INSERT INTO portfolios (name, description) VALUES (?1, ?2)",
            params![portfolio.name, portfolio.description],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Portfolio> {
        let mut stmt = conn.prepare(
            "SELECT id, name, description, created_at, updated_at FROM portfolios WHERE id = ?1"
        )?;
        
        stmt.query_row(params![id], |row| {
            Ok(Portfolio {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                description: row.get(2)?,
                created_at: Some(row.get(3)?),
                updated_at: Some(row.get(4)?),
            })
        })
    }

    pub fn get_all(conn: &Connection) -> Result<Vec<Portfolio>> {
        let mut stmt = conn.prepare(
            "SELECT id, name, description, created_at, updated_at FROM portfolios ORDER BY created_at DESC"
        )?;
        
        let portfolios = stmt.query_map([], |row| {
            Ok(Portfolio {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                description: row.get(2)?,
                created_at: Some(row.get(3)?),
                updated_at: Some(row.get(4)?),
            })
        })?;

        portfolios.collect()
    }

    pub fn update(conn: &Connection, portfolio: &Portfolio) -> Result<()> {
        conn.execute(
            "UPDATE portfolios SET name = ?1, description = ?2 WHERE id = ?3",
            params![portfolio.name, portfolio.description, portfolio.id],
        )?;
        Ok(())
    }

    pub fn delete(conn: &Connection, id: i64) -> Result<()> {
        conn.execute("DELETE FROM portfolios WHERE id = ?1", params![id])?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_db;
    use tempfile::NamedTempFile;

    fn setup_test_db() -> Connection {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_path_buf();
        init_db(&db_path).unwrap()
    }

    #[test]
    fn test_create_portfolio() {
        let conn = setup_test_db();
        let portfolio = Portfolio::new("Test Portfolio".to_string(), Some("Test Description".to_string()));
        
        let id = PortfolioService::create(&conn, &portfolio).unwrap();
        
        assert!(id > 0);
    }

    #[test]
    fn test_get_portfolio_by_id() {
        let conn = setup_test_db();
        let portfolio = Portfolio::new("Test Portfolio".to_string(), Some("Test Description".to_string()));
        let id = PortfolioService::create(&conn, &portfolio).unwrap();
        
        let retrieved = PortfolioService::get_by_id(&conn, id).unwrap();
        
        assert_eq!(retrieved.name, "Test Portfolio");
        assert_eq!(retrieved.description, Some("Test Description".to_string()));
        assert_eq!(retrieved.id, Some(id));
    }

    #[test]
    fn test_get_all_portfolios() {
        let conn = setup_test_db();
        let p1 = Portfolio::new("Portfolio 1".to_string(), None);
        let p2 = Portfolio::new("Portfolio 2".to_string(), None);
        
        PortfolioService::create(&conn, &p1).unwrap();
        PortfolioService::create(&conn, &p2).unwrap();
        
        let all = PortfolioService::get_all(&conn).unwrap();
        
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn test_update_portfolio() {
        let conn = setup_test_db();
        let portfolio = Portfolio::new("Original Name".to_string(), None);
        let id = PortfolioService::create(&conn, &portfolio).unwrap();
        
        let mut updated = PortfolioService::get_by_id(&conn, id).unwrap();
        updated.name = "Updated Name".to_string();
        updated.description = Some("New Description".to_string());
        
        PortfolioService::update(&conn, &updated).unwrap();
        
        let retrieved = PortfolioService::get_by_id(&conn, id).unwrap();
        assert_eq!(retrieved.name, "Updated Name");
        assert_eq!(retrieved.description, Some("New Description".to_string()));
    }

    #[test]
    fn test_delete_portfolio() {
        let conn = setup_test_db();
        let portfolio = Portfolio::new("To Delete".to_string(), None);
        let id = PortfolioService::create(&conn, &portfolio).unwrap();
        
        PortfolioService::delete(&conn, id).unwrap();
        
        let result = PortfolioService::get_by_id(&conn, id);
        assert!(result.is_err());
    }
}
