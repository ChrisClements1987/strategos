use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Feature {
    pub id: Option<i64>,
    pub product_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl Feature {
    pub fn new(
        product_id: i64,
        name: String,
        description: Option<String>,
    ) -> Self {
        Feature {
            id: None,
            product_id,
            name,
            description,
            status: "planned".to_string(),
            priority: "medium".to_string(),
            created_at: None,
            updated_at: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_new() {
        let feature = Feature::new(1, "Test Feature".to_string(), Some("Description".to_string()));

        assert_eq!(feature.product_id, 1);
        assert_eq!(feature.name, "Test Feature");
        assert_eq!(feature.description, Some("Description".to_string()));
        assert_eq!(feature.status, "planned");
        assert_eq!(feature.priority, "medium");
    }
}
