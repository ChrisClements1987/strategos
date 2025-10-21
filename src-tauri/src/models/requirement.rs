use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Requirement {
    pub id: Option<i64>,
    pub client_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub priority: String,
    pub status: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl Requirement {
    pub fn new(client_id: i64, name: String, description: Option<String>) -> Self {
        Requirement {
            id: None,
            client_id,
            name,
            description,
            priority: "medium".to_string(),
            status: "pending".to_string(),
            created_at: None,
            updated_at: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_requirement_new() {
        let req = Requirement::new(1, "Test Requirement".to_string(), Some("Description".to_string()));

        assert_eq!(req.client_id, 1);
        assert_eq!(req.name, "Test Requirement");
        assert_eq!(req.description, Some("Description".to_string()));
        assert_eq!(req.priority, "medium");
        assert_eq!(req.status, "pending");
    }
}
