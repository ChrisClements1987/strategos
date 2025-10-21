use crate::models::Product;
use rusqlite::{Connection, Result, params};

pub struct ProductService;

impl ProductService {
    pub fn create(conn: &Connection, product: &Product) -> Result<i64> {
        conn.execute(
            "INSERT INTO products (portfolio_id, parent_product_id, name, description, product_type, status) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                product.portfolio_id,
                product.parent_product_id,
                product.name,
                product.description,
                product.product_type,
                product.status
            ],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Product> {
        let mut stmt = conn.prepare(
            "SELECT id, portfolio_id, parent_product_id, name, description, product_type, status, created_at, updated_at 
             FROM products WHERE id = ?1"
        )?;
        
        stmt.query_row(params![id], |row| {
            Ok(Product {
                id: Some(row.get(0)?),
                portfolio_id: row.get(1)?,
                parent_product_id: row.get(2)?,
                name: row.get(3)?,
                description: row.get(4)?,
                product_type: row.get(5)?,
                status: row.get(6)?,
                created_at: Some(row.get(7)?),
                updated_at: Some(row.get(8)?),
            })
        })
    }

    pub fn get_by_portfolio(conn: &Connection, portfolio_id: i64) -> Result<Vec<Product>> {
        let mut stmt = conn.prepare(
            "SELECT id, portfolio_id, parent_product_id, name, description, product_type, status, created_at, updated_at 
             FROM products WHERE portfolio_id = ?1 ORDER BY created_at DESC"
        )?;
        
        let products = stmt.query_map(params![portfolio_id], |row| {
            Ok(Product {
                id: Some(row.get(0)?),
                portfolio_id: row.get(1)?,
                parent_product_id: row.get(2)?,
                name: row.get(3)?,
                description: row.get(4)?,
                product_type: row.get(5)?,
                status: row.get(6)?,
                created_at: Some(row.get(7)?),
                updated_at: Some(row.get(8)?),
            })
        })?;

        products.collect()
    }

    pub fn get_modules(conn: &Connection, parent_product_id: i64) -> Result<Vec<Product>> {
        let mut stmt = conn.prepare(
            "SELECT id, portfolio_id, parent_product_id, name, description, product_type, status, created_at, updated_at 
             FROM products WHERE parent_product_id = ?1 ORDER BY created_at DESC"
        )?;
        
        let products = stmt.query_map(params![parent_product_id], |row| {
            Ok(Product {
                id: Some(row.get(0)?),
                portfolio_id: row.get(1)?,
                parent_product_id: row.get(2)?,
                name: row.get(3)?,
                description: row.get(4)?,
                product_type: row.get(5)?,
                status: row.get(6)?,
                created_at: Some(row.get(7)?),
                updated_at: Some(row.get(8)?),
            })
        })?;

        products.collect()
    }

    pub fn update(conn: &Connection, product: &Product) -> Result<()> {
        conn.execute(
            "UPDATE products SET name = ?1, description = ?2, product_type = ?3, status = ?4 WHERE id = ?5",
            params![product.name, product.description, product.product_type, product.status, product.id],
        )?;
        Ok(())
    }

    pub fn delete(conn: &Connection, id: i64) -> Result<()> {
        conn.execute("DELETE FROM products WHERE id = ?1", params![id])?;
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
        
        // Create a test portfolio
        let portfolio = Portfolio::new("Test Portfolio".to_string(), None);
        let portfolio_id = PortfolioService::create(&conn, &portfolio).unwrap();
        
        (conn, portfolio_id)
    }

    #[test]
    fn test_create_product() {
        let (conn, portfolio_id) = setup_test_db();
        let product = Product::new(portfolio_id, "Test Product".to_string(), Some("Description".to_string()), None);
        
        let id = ProductService::create(&conn, &product).unwrap();
        
        assert!(id > 0);
    }

    #[test]
    fn test_get_product_by_id() {
        let (conn, portfolio_id) = setup_test_db();
        let product = Product::new(portfolio_id, "Test Product".to_string(), Some("Description".to_string()), None);
        let id = ProductService::create(&conn, &product).unwrap();
        
        let retrieved = ProductService::get_by_id(&conn, id).unwrap();
        
        assert_eq!(retrieved.name, "Test Product");
        assert_eq!(retrieved.description, Some("Description".to_string()));
        assert_eq!(retrieved.portfolio_id, portfolio_id);
    }

    #[test]
    fn test_get_products_by_portfolio() {
        let (conn, portfolio_id) = setup_test_db();
        let p1 = Product::new(portfolio_id, "Product 1".to_string(), None, None);
        let p2 = Product::new(portfolio_id, "Product 2".to_string(), None, None);
        
        ProductService::create(&conn, &p1).unwrap();
        ProductService::create(&conn, &p2).unwrap();
        
        let all = ProductService::get_by_portfolio(&conn, portfolio_id).unwrap();
        
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn test_create_module() {
        let (conn, portfolio_id) = setup_test_db();
        let product = Product::new(portfolio_id, "Main Product".to_string(), None, None);
        let product_id = ProductService::create(&conn, &product).unwrap();
        
        let module = Product::new_module(portfolio_id, product_id, "Module 1".to_string(), None);
        let module_id = ProductService::create(&conn, &module).unwrap();
        
        let retrieved = ProductService::get_by_id(&conn, module_id).unwrap();
        assert_eq!(retrieved.product_type, "module");
        assert_eq!(retrieved.parent_product_id, Some(product_id));
    }

    #[test]
    fn test_get_modules() {
        let (conn, portfolio_id) = setup_test_db();
        let product = Product::new(portfolio_id, "Main Product".to_string(), None, None);
        let product_id = ProductService::create(&conn, &product).unwrap();
        
        let m1 = Product::new_module(portfolio_id, product_id, "Module 1".to_string(), None);
        let m2 = Product::new_module(portfolio_id, product_id, "Module 2".to_string(), None);
        
        ProductService::create(&conn, &m1).unwrap();
        ProductService::create(&conn, &m2).unwrap();
        
        let modules = ProductService::get_modules(&conn, product_id).unwrap();
        
        assert_eq!(modules.len(), 2);
    }

    #[test]
    fn test_update_product() {
        let (conn, portfolio_id) = setup_test_db();
        let product = Product::new(portfolio_id, "Original Name".to_string(), None, None);
        let id = ProductService::create(&conn, &product).unwrap();
        
        let mut updated = ProductService::get_by_id(&conn, id).unwrap();
        updated.name = "Updated Name".to_string();
        updated.status = "inactive".to_string();
        
        ProductService::update(&conn, &updated).unwrap();
        
        let retrieved = ProductService::get_by_id(&conn, id).unwrap();
        assert_eq!(retrieved.name, "Updated Name");
        assert_eq!(retrieved.status, "inactive");
    }

    #[test]
    fn test_delete_product() {
        let (conn, portfolio_id) = setup_test_db();
        let product = Product::new(portfolio_id, "To Delete".to_string(), None, None);
        let id = ProductService::create(&conn, &product).unwrap();
        
        ProductService::delete(&conn, id).unwrap();
        
        let result = ProductService::get_by_id(&conn, id);
        assert!(result.is_err());
    }
}
