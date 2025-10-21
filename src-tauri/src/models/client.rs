use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Client {
    pub id: Option<i64>,
    pub portfolio_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub client_type: String,
    pub status: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl Client {
    pub fn new(portfolio_id: i64, name: String, description: Option<String>) -> Self {
        Client {
            id: None,
            portfolio_id,
            name,
            description,
            client_type: "persona".to_string(),
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
    fn test_client_new() {
        let client = Client::new(1, "Test Client".to_string(), Some("Description".to_string()));

        assert_eq!(client.portfolio_id, 1);
        assert_eq!(client.name, "Test Client");
        assert_eq!(client.description, Some("Description".to_string()));
        assert_eq!(client.client_type, "persona");
        assert_eq!(client.status, "active");
    }
}
