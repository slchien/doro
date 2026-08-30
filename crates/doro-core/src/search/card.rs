use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolCard {
    pub name: String,
    pub server: String,
    pub description: String,
    pub parameters: Vec<String>,
}

impl ToolCard {
    pub fn new(
        name: impl Into<String>,
        server: impl Into<String>,
        description: impl Into<String>,
        parameters: impl Into<Vec<String>>,
    ) -> Self {
        Self {
            name: name.into(),
            server: server.into(),
            description: description.into(),
            parameters: parameters.into(),
        }
    }

    pub fn searchable_text(&self) -> String {
        format!(
            "{} {} {} {}",
            self.server,
            self.name,
            self.description,
            self.parameters.join(" ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_card_new() {
        let card = ToolCard::new(
            "test".to_string(),
            "test".to_string(),
            "test".to_string(),
            vec!["test".to_string()],
        );
        assert_eq!(card.name, "test");
        assert_eq!(card.server, "test");
        assert_eq!(card.description, "test");
        assert_eq!(card.parameters, vec!["test".to_string()]);
    }

    #[test]
    fn test_searchable_text_base() {
        let card = ToolCard::new(
            "test".to_string(),
            "test".to_string(),
            "test".to_string(),
            vec!["test".to_string()],
        );
        assert_eq!(card.searchable_text(), "test test test test");
    }
}
