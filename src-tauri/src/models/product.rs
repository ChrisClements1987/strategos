use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Product {
    pub id: Option<i64>,
    pub portfolio_id: i64,
    pub parent_product_id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub product_type: String,
    pub status: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl Product {
    pub fn new(
        portfolio_id: i64,
        name: String,
        description: Option<String>,
        parent_product_id: Option<i64>,
    ) -> Self {
        Product {
            id: None,
            portfolio_id,
            parent_product_id,
            name,
            description,
            product_type: "product".to_string(),
            status: "active".to_string(),
            created_at: None,
            updated_at: None,
        }
    }

    pub fn new_module(
        portfolio_id: i64,
        parent_product_id: i64,
        name: String,
        description: Option<String>,
    ) -> Self {
        Product {
            id: None,
            portfolio_id,
            parent_product_id: Some(parent_product_id),
            name,
            description,
            product_type: "module".to_string(),
            status: "active".to_string(),
            created_at: None,
            updated_at: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_new() {
        let product = Product::new(1, "Test Product".to_string(), Some("Description".to_string()), None);

        assert_eq!(product.portfolio_id, 1);
        assert_eq!(product.name, "Test Product");
        assert_eq!(product.description, Some("Description".to_string()));
        assert_eq!(product.product_type, "product");
        assert_eq!(product.status, "active");
        assert!(product.parent_product_id.is_none());
    }

    #[test]
    fn test_product_new_module() {
        let product = Product::new_module(1, 5, "Test Module".to_string(), None);

        assert_eq!(product.portfolio_id, 1);
        assert_eq!(product.parent_product_id, Some(5));
        assert_eq!(product.name, "Test Module");
        assert_eq!(product.product_type, "module");
    }
}
