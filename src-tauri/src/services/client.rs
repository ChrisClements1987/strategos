use crate::models::Client;
use rusqlite::{Connection, Result, params};

pub struct ClientService;

impl ClientService {
    pub fn create(conn: &Connection, client: &Client) -> Result<i64> {
        conn.execute(
            "INSERT INTO clients (portfolio_id, name, description, client_type, status) 
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                client.portfolio_id,
                client.name,
                client.description,
                client.client_type,
                client.status
            ],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Client> {
        let mut stmt = conn.prepare(
            "SELECT id, portfolio_id, name, description, client_type, status, created_at, updated_at 
             FROM clients WHERE id = ?1"
        )?;
        
        stmt.query_row(params![id], |row| {
            Ok(Client {
                id: Some(row.get(0)?),
                portfolio_id: row.get(1)?,
                name: row.get(2)?,
                description: row.get(3)?,
                client_type: row.get(4)?,
                status: row.get(5)?,
                created_at: Some(row.get(6)?),
                updated_at: Some(row.get(7)?),
            })
        })
    }

    pub fn get_by_portfolio(conn: &Connection, portfolio_id: i64) -> Result<Vec<Client>> {
        let mut stmt = conn.prepare(
            "SELECT id, portfolio_id, name, description, client_type, status, created_at, updated_at 
             FROM clients WHERE portfolio_id = ?1 ORDER BY created_at DESC"
        )?;
        
        let clients = stmt.query_map(params![portfolio_id], |row| {
            Ok(Client {
                id: Some(row.get(0)?),
                portfolio_id: row.get(1)?,
                name: row.get(2)?,
                description: row.get(3)?,
                client_type: row.get(4)?,
                status: row.get(5)?,
                created_at: Some(row.get(6)?),
                updated_at: Some(row.get(7)?),
            })
        })?;

        clients.collect()
    }

    pub fn update(conn: &Connection, client: &Client) -> Result<()> {
        conn.execute(
            "UPDATE clients SET name = ?1, description = ?2, client_type = ?3, status = ?4 WHERE id = ?5",
            params![client.name, client.description, client.client_type, client.status, client.id],
        )?;
        Ok(())
    }

    pub fn delete(conn: &Connection, id: i64) -> Result<()> {
        conn.execute("DELETE FROM clients WHERE id = ?1", params![id])?;
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
        
        (conn, portfolio_id)
    }

    #[test]
    fn test_create_client() {
        let (conn, portfolio_id) = setup_test_db();
        let client = Client::new(portfolio_id, "Test Client".to_string(), Some("Description".to_string()));
        
        let id = ClientService::create(&conn, &client).unwrap();
        
        assert!(id > 0);
    }

    #[test]
    fn test_get_client_by_id() {
        let (conn, portfolio_id) = setup_test_db();
        let client = Client::new(portfolio_id, "Test Client".to_string(), Some("Description".to_string()));
        let id = ClientService::create(&conn, &client).unwrap();
        
        let retrieved = ClientService::get_by_id(&conn, id).unwrap();
        
        assert_eq!(retrieved.name, "Test Client");
        assert_eq!(retrieved.description, Some("Description".to_string()));
    }

    #[test]
    fn test_get_clients_by_portfolio() {
        let (conn, portfolio_id) = setup_test_db();
        let c1 = Client::new(portfolio_id, "Client 1".to_string(), None);
        let c2 = Client::new(portfolio_id, "Client 2".to_string(), None);
        
        ClientService::create(&conn, &c1).unwrap();
        ClientService::create(&conn, &c2).unwrap();
        
        let all = ClientService::get_by_portfolio(&conn, portfolio_id).unwrap();
        
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn test_update_client() {
        let (conn, portfolio_id) = setup_test_db();
        let client = Client::new(portfolio_id, "Original".to_string(), None);
        let id = ClientService::create(&conn, &client).unwrap();
        
        let mut updated = ClientService::get_by_id(&conn, id).unwrap();
        updated.name = "Updated".to_string();
        updated.client_type = "enterprise".to_string();
        
        ClientService::update(&conn, &updated).unwrap();
        
        let retrieved = ClientService::get_by_id(&conn, id).unwrap();
        assert_eq!(retrieved.name, "Updated");
        assert_eq!(retrieved.client_type, "enterprise");
    }

    #[test]
    fn test_delete_client() {
        let (conn, portfolio_id) = setup_test_db();
        let client = Client::new(portfolio_id, "To Delete".to_string(), None);
        let id = ClientService::create(&conn, &client).unwrap();
        
        ClientService::delete(&conn, id).unwrap();
        
        let result = ClientService::get_by_id(&conn, id);
        assert!(result.is_err());
    }
}
