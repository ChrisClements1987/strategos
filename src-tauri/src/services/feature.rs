use crate::models::Feature;
use rusqlite::{Connection, Result, params};

pub struct FeatureService;

impl FeatureService {
    pub fn create(conn: &Connection, feature: &Feature) -> Result<i64> {
        conn.execute(
            "INSERT INTO features (product_id, name, description, status, priority) 
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                feature.product_id,
                feature.name,
                feature.description,
                feature.status,
                feature.priority
            ],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Feature> {
        let mut stmt = conn.prepare(
            "SELECT id, product_id, name, description, status, priority, created_at, updated_at 
             FROM features WHERE id = ?1"
        )?;
        
        stmt.query_row(params![id], |row| {
            Ok(Feature {
                id: Some(row.get(0)?),
                product_id: row.get(1)?,
                name: row.get(2)?,
                description: row.get(3)?,
                status: row.get(4)?,
                priority: row.get(5)?,
                created_at: Some(row.get(6)?),
                updated_at: Some(row.get(7)?),
            })
        })
    }

    pub fn get_by_product(conn: &Connection, product_id: i64) -> Result<Vec<Feature>> {
        let mut stmt = conn.prepare(
            "SELECT id, product_id, name, description, status, priority, created_at, updated_at 
             FROM features WHERE product_id = ?1 ORDER BY created_at DESC"
        )?;
        
        let features = stmt.query_map(params![product_id], |row| {
            Ok(Feature {
                id: Some(row.get(0)?),
                product_id: row.get(1)?,
                name: row.get(2)?,
                description: row.get(3)?,
                status: row.get(4)?,
                priority: row.get(5)?,
                created_at: Some(row.get(6)?),
                updated_at: Some(row.get(7)?),
            })
        })?;

        features.collect()
    }

    pub fn update(conn: &Connection, feature: &Feature) -> Result<()> {
        conn.execute(
            "UPDATE features SET name = ?1, description = ?2, status = ?3, priority = ?4 WHERE id = ?5",
            params![feature.name, feature.description, feature.status, feature.priority, feature.id],
        )?;
        Ok(())
    }

    pub fn delete(conn: &Connection, id: i64) -> Result<()> {
        conn.execute("DELETE FROM features WHERE id = ?1", params![id])?;
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

    fn setup_test_db() -> (Connection, i64) {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_path_buf();
        let conn = init_db(&db_path).unwrap();
        
        // Create test portfolio and product directly
        let portfolio = Portfolio::new("Test Portfolio".to_string(), None);
        let portfolio_id = PortfolioService::create(&conn, &portfolio).unwrap();
        
        // Create product directly via SQL since Product model isn't in develop yet
        conn.execute(
            "INSERT INTO products (portfolio_id, name, product_type, status) VALUES (?1, ?2, ?3, ?4)",
            params![portfolio_id, "Test Product", "product", "active"],
        ).unwrap();
        let product_id = conn.last_insert_rowid();
        
        (conn, product_id)
    }

    #[test]
    fn test_create_feature() {
        let (conn, product_id) = setup_test_db();
        let feature = Feature::new(product_id, "Test Feature".to_string(), Some("Description".to_string()));
        
        let id = FeatureService::create(&conn, &feature).unwrap();
        
        assert!(id > 0);
    }

    #[test]
    fn test_get_feature_by_id() {
        let (conn, product_id) = setup_test_db();
        let feature = Feature::new(product_id, "Test Feature".to_string(), Some("Description".to_string()));
        let id = FeatureService::create(&conn, &feature).unwrap();
        
        let retrieved = FeatureService::get_by_id(&conn, id).unwrap();
        
        assert_eq!(retrieved.name, "Test Feature");
        assert_eq!(retrieved.description, Some("Description".to_string()));
        assert_eq!(retrieved.product_id, product_id);
    }

    #[test]
    fn test_get_features_by_product() {
        let (conn, product_id) = setup_test_db();
        let f1 = Feature::new(product_id, "Feature 1".to_string(), None);
        let f2 = Feature::new(product_id, "Feature 2".to_string(), None);
        
        FeatureService::create(&conn, &f1).unwrap();
        FeatureService::create(&conn, &f2).unwrap();
        
        let all = FeatureService::get_by_product(&conn, product_id).unwrap();
        
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn test_update_feature() {
        let (conn, product_id) = setup_test_db();
        let feature = Feature::new(product_id, "Original Name".to_string(), None);
        let id = FeatureService::create(&conn, &feature).unwrap();
        
        let mut updated = FeatureService::get_by_id(&conn, id).unwrap();
        updated.name = "Updated Name".to_string();
        updated.status = "in_progress".to_string();
        updated.priority = "high".to_string();
        
        FeatureService::update(&conn, &updated).unwrap();
        
        let retrieved = FeatureService::get_by_id(&conn, id).unwrap();
        assert_eq!(retrieved.name, "Updated Name");
        assert_eq!(retrieved.status, "in_progress");
        assert_eq!(retrieved.priority, "high");
    }

    #[test]
    fn test_delete_feature() {
        let (conn, product_id) = setup_test_db();
        let feature = Feature::new(product_id, "To Delete".to_string(), None);
        let id = FeatureService::create(&conn, &feature).unwrap();
        
        FeatureService::delete(&conn, id).unwrap();
        
        let result = FeatureService::get_by_id(&conn, id);
        assert!(result.is_err());
    }
}
