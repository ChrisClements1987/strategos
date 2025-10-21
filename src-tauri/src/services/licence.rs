use crate::models::Licence;
use rusqlite::{Connection, Result, params};

pub struct LicenceService;

impl LicenceService {
    pub fn create(conn: &Connection, licence: &Licence) -> Result<i64> {
        conn.execute(
            "INSERT INTO licences (portfolio_id, name, description, licence_type, status) 
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                licence.portfolio_id,
                licence.name,
                licence.description,
                licence.licence_type,
                licence.status
            ],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Licence> {
        let mut stmt = conn.prepare(
            "SELECT id, portfolio_id, name, description, licence_type, status, created_at, updated_at 
             FROM licences WHERE id = ?1"
        )?;
        
        stmt.query_row(params![id], |row| {
            Ok(Licence {
                id: Some(row.get(0)?),
                portfolio_id: row.get(1)?,
                name: row.get(2)?,
                description: row.get(3)?,
                licence_type: row.get(4)?,
                status: row.get(5)?,
                created_at: Some(row.get(6)?),
                updated_at: Some(row.get(7)?),
            })
        })
    }

    pub fn get_by_portfolio(conn: &Connection, portfolio_id: i64) -> Result<Vec<Licence>> {
        let mut stmt = conn.prepare(
            "SELECT id, portfolio_id, name, description, licence_type, status, created_at, updated_at 
             FROM licences WHERE portfolio_id = ?1 ORDER BY created_at DESC"
        )?;
        
        let licences = stmt.query_map(params![portfolio_id], |row| {
            Ok(Licence {
                id: Some(row.get(0)?),
                portfolio_id: row.get(1)?,
                name: row.get(2)?,
                description: row.get(3)?,
                licence_type: row.get(4)?,
                status: row.get(5)?,
                created_at: Some(row.get(6)?),
                updated_at: Some(row.get(7)?),
            })
        })?;

        licences.collect()
    }

    pub fn add_feature(conn: &Connection, licence_id: i64, feature_id: i64) -> Result<()> {
        conn.execute(
            "INSERT OR IGNORE INTO licence_features (licence_id, feature_id) VALUES (?1, ?2)",
            params![licence_id, feature_id],
        )?;
        Ok(())
    }

    pub fn remove_feature(conn: &Connection, licence_id: i64, feature_id: i64) -> Result<()> {
        conn.execute(
            "DELETE FROM licence_features WHERE licence_id = ?1 AND feature_id = ?2",
            params![licence_id, feature_id],
        )?;
        Ok(())
    }

    pub fn get_features(conn: &Connection, licence_id: i64) -> Result<Vec<i64>> {
        let mut stmt = conn.prepare(
            "SELECT feature_id FROM licence_features WHERE licence_id = ?1"
        )?;
        
        let feature_ids = stmt.query_map(params![licence_id], |row| row.get(0))?;
        feature_ids.collect()
    }

    pub fn update(conn: &Connection, licence: &Licence) -> Result<()> {
        conn.execute(
            "UPDATE licences SET name = ?1, description = ?2, licence_type = ?3, status = ?4 WHERE id = ?5",
            params![licence.name, licence.description, licence.licence_type, licence.status, licence.id],
        )?;
        Ok(())
    }

    pub fn delete(conn: &Connection, id: i64) -> Result<()> {
        conn.execute("DELETE FROM licences WHERE id = ?1", params![id])?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_db;
    use crate::services::portfolio::PortfolioService;
    use crate::models::Portfolio;
    use tempfile::NamedTempFile;

    fn setup_test_db() -> (Connection, i64, i64) {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_path_buf();
        let conn = init_db(&db_path).unwrap();
        
        // Create test portfolio
        let portfolio = Portfolio::new("Test Portfolio".to_string(), None);
        let portfolio_id = PortfolioService::create(&conn, &portfolio).unwrap();
        
        // Create test product and feature
        conn.execute(
            "INSERT INTO products (portfolio_id, name) VALUES (?1, ?2)",
            params![portfolio_id, "Test Product"],
        ).unwrap();
        let product_id = conn.last_insert_rowid();
        
        conn.execute(
            "INSERT INTO features (product_id, name) VALUES (?1, ?2)",
            params![product_id, "Test Feature"],
        ).unwrap();
        let feature_id = conn.last_insert_rowid();
        
        (conn, portfolio_id, feature_id)
    }

    #[test]
    fn test_create_licence() {
        let (conn, portfolio_id, _) = setup_test_db();
        let licence = Licence::new(portfolio_id, "Test Licence".to_string(), Some("Description".to_string()));
        
        let id = LicenceService::create(&conn, &licence).unwrap();
        
        assert!(id > 0);
    }

    #[test]
    fn test_get_licence_by_id() {
        let (conn, portfolio_id, _) = setup_test_db();
        let licence = Licence::new(portfolio_id, "Test Licence".to_string(), Some("Description".to_string()));
        let id = LicenceService::create(&conn, &licence).unwrap();
        
        let retrieved = LicenceService::get_by_id(&conn, id).unwrap();
        
        assert_eq!(retrieved.name, "Test Licence");
        assert_eq!(retrieved.description, Some("Description".to_string()));
    }

    #[test]
    fn test_get_licences_by_portfolio() {
        let (conn, portfolio_id, _) = setup_test_db();
        let l1 = Licence::new(portfolio_id, "Licence 1".to_string(), None);
        let l2 = Licence::new(portfolio_id, "Licence 2".to_string(), None);
        
        LicenceService::create(&conn, &l1).unwrap();
        LicenceService::create(&conn, &l2).unwrap();
        
        let all = LicenceService::get_by_portfolio(&conn, portfolio_id).unwrap();
        
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn test_add_feature_to_licence() {
        let (conn, portfolio_id, feature_id) = setup_test_db();
        let licence = Licence::new(portfolio_id, "Test Licence".to_string(), None);
        let licence_id = LicenceService::create(&conn, &licence).unwrap();
        
        LicenceService::add_feature(&conn, licence_id, feature_id).unwrap();
        
        let features = LicenceService::get_features(&conn, licence_id).unwrap();
        assert_eq!(features.len(), 1);
        assert_eq!(features[0], feature_id);
    }

    #[test]
    fn test_remove_feature_from_licence() {
        let (conn, portfolio_id, feature_id) = setup_test_db();
        let licence = Licence::new(portfolio_id, "Test Licence".to_string(), None);
        let licence_id = LicenceService::create(&conn, &licence).unwrap();
        
        LicenceService::add_feature(&conn, licence_id, feature_id).unwrap();
        LicenceService::remove_feature(&conn, licence_id, feature_id).unwrap();
        
        let features = LicenceService::get_features(&conn, licence_id).unwrap();
        assert_eq!(features.len(), 0);
    }

    #[test]
    fn test_update_licence() {
        let (conn, portfolio_id, _) = setup_test_db();
        let licence = Licence::new(portfolio_id, "Original".to_string(), None);
        let id = LicenceService::create(&conn, &licence).unwrap();
        
        let mut updated = LicenceService::get_by_id(&conn, id).unwrap();
        updated.name = "Updated".to_string();
        updated.licence_type = "enterprise".to_string();
        
        LicenceService::update(&conn, &updated).unwrap();
        
        let retrieved = LicenceService::get_by_id(&conn, id).unwrap();
        assert_eq!(retrieved.name, "Updated");
        assert_eq!(retrieved.licence_type, "enterprise");
    }

    #[test]
    fn test_delete_licence() {
        let (conn, portfolio_id, _) = setup_test_db();
        let licence = Licence::new(portfolio_id, "To Delete".to_string(), None);
        let id = LicenceService::create(&conn, &licence).unwrap();
        
        LicenceService::delete(&conn, id).unwrap();
        
        let result = LicenceService::get_by_id(&conn, id);
        assert!(result.is_err());
    }
}
