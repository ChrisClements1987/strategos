use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Portfolio {
    pub id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl Portfolio {
    pub fn new(name: String, description: Option<String>) -> Self {
        Portfolio {
            id: None,
            name,
            description,
            created_at: None,
            updated_at: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_portfolio_new() {
        let portfolio = Portfolio::new("Test Portfolio".to_string(), Some("Description".to_string()));
        
        assert_eq!(portfolio.name, "Test Portfolio");
        assert_eq!(portfolio.description, Some("Description".to_string()));
        assert!(portfolio.id.is_none());
        assert!(portfolio.created_at.is_none());
    }

    #[test]
    fn test_portfolio_new_without_description() {
        let portfolio = Portfolio::new("Test Portfolio".to_string(), None);
        
        assert_eq!(portfolio.name, "Test Portfolio");
        assert!(portfolio.description.is_none());
    }
}
