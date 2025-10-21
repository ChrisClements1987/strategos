use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Licence {
    pub id: Option<i64>,
    pub portfolio_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub licence_type: String,
    pub status: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl Licence {
    pub fn new(portfolio_id: i64, name: String, description: Option<String>) -> Self {
        Licence {
            id: None,
            portfolio_id,
            name,
            description,
            licence_type: "standard".to_string(),
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
    fn test_licence_new() {
        let licence = Licence::new(1, "Test Licence".to_string(), Some("Description".to_string()));

        assert_eq!(licence.portfolio_id, 1);
        assert_eq!(licence.name, "Test Licence");
        assert_eq!(licence.description, Some("Description".to_string()));
        assert_eq!(licence.licence_type, "standard");
        assert_eq!(licence.status, "active");
    }
}
