use crate::models::Requirement;
use rusqlite::{Connection, Result, params};

pub struct RequirementService;

impl RequirementService {
    pub fn create(conn: &Connection, requirement: &Requirement) -> Result<i64> {
        conn.execute(
            "INSERT INTO requirements (client_id, name, description, priority, status) 
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                requirement.client_id,
                requirement.name,
                requirement.description,
                requirement.priority,
                requirement.status
            ],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Requirement> {
        let mut stmt = conn.prepare(
            "SELECT id, client_id, name, description, priority, status, created_at, updated_at 
             FROM requirements WHERE id = ?1"
        )?;
        
        stmt.query_row(params![id], |row| {
            Ok(Requirement {
                id: Some(row.get(0)?),
                client_id: row.get(1)?,
                name: row.get(2)?,
                description: row.get(3)?,
                priority: row.get(4)?,
                status: row.get(5)?,
                created_at: Some(row.get(6)?),
                updated_at: Some(row.get(7)?),
            })
        })
    }

    pub fn get_by_client(conn: &Connection, client_id: i64) -> Result<Vec<Requirement>> {
        let mut stmt = conn.prepare(
            "SELECT id, client_id, name, description, priority, status, created_at, updated_at 
             FROM requirements WHERE client_id = ?1 ORDER BY created_at DESC"
        )?;
        
        let requirements = stmt.query_map(params![client_id], |row| {
            Ok(Requirement {
                id: Some(row.get(0)?),
                client_id: row.get(1)?,
                name: row.get(2)?,
                description: row.get(3)?,
                priority: row.get(4)?,
                status: row.get(5)?,
                created_at: Some(row.get(6)?),
                updated_at: Some(row.get(7)?),
            })
        })?;

        requirements.collect()
    }

    pub fn update(conn: &Connection, requirement: &Requirement) -> Result<()> {
        conn.execute(
            "UPDATE requirements SET name = ?1, description = ?2, priority = ?3, status = ?4 WHERE id = ?5",
            params![requirement.name, requirement.description, requirement.priority, requirement.status, requirement.id],
        )?;
        Ok(())
    }

    pub fn delete(conn: &Connection, id: i64) -> Result<()> {
        conn.execute("DELETE FROM requirements WHERE id = ?1", params![id])?;
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
        
        let portfolio = Portfolio::new("Test Portfolio".to_string(), None);
        let portfolio_id = PortfolioService::create(&conn, &portfolio).unwrap();
        
        conn.execute(
            "INSERT INTO clients (portfolio_id, name) VALUES (?1, ?2)",
            params![portfolio_id, "Test Client"],
        ).unwrap();
        let client_id = conn.last_insert_rowid();
        
        (conn, client_id)
    }

    #[test]
    fn test_create_requirement() {
        let (conn, client_id) = setup_test_db();
        let req = Requirement::new(client_id, "Test Requirement".to_string(), Some("Description".to_string()));
        
        let id = RequirementService::create(&conn, &req).unwrap();
        
        assert!(id > 0);
    }

    #[test]
    fn test_get_requirement_by_id() {
        let (conn, client_id) = setup_test_db();
        let req = Requirement::new(client_id, "Test Requirement".to_string(), Some("Description".to_string()));
        let id = RequirementService::create(&conn, &req).unwrap();
        
        let retrieved = RequirementService::get_by_id(&conn, id).unwrap();
        
        assert_eq!(retrieved.name, "Test Requirement");
        assert_eq!(retrieved.description, Some("Description".to_string()));
    }

    #[test]
    fn test_get_requirements_by_client() {
        let (conn, client_id) = setup_test_db();
        let r1 = Requirement::new(client_id, "Requirement 1".to_string(), None);
        let r2 = Requirement::new(client_id, "Requirement 2".to_string(), None);
        
        RequirementService::create(&conn, &r1).unwrap();
        RequirementService::create(&conn, &r2).unwrap();
        
        let all = RequirementService::get_by_client(&conn, client_id).unwrap();
        
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn test_update_requirement() {
        let (conn, client_id) = setup_test_db();
        let req = Requirement::new(client_id, "Original".to_string(), None);
        let id = RequirementService::create(&conn, &req).unwrap();
        
        let mut updated = RequirementService::get_by_id(&conn, id).unwrap();
        updated.name = "Updated".to_string();
        updated.priority = "high".to_string();
        updated.status = "approved".to_string();
        
        RequirementService::update(&conn, &updated).unwrap();
        
        let retrieved = RequirementService::get_by_id(&conn, id).unwrap();
        assert_eq!(retrieved.name, "Updated");
        assert_eq!(retrieved.priority, "high");
        assert_eq!(retrieved.status, "approved");
    }

    #[test]
    fn test_delete_requirement() {
        let (conn, client_id) = setup_test_db();
        let req = Requirement::new(client_id, "To Delete".to_string(), None);
        let id = RequirementService::create(&conn, &req).unwrap();
        
        RequirementService::delete(&conn, id).unwrap();
        
        let result = RequirementService::get_by_id(&conn, id);
        assert!(result.is_err());
    }
}
