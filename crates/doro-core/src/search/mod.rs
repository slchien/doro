pub mod bm25;
pub mod card;
pub mod tokenizer;

pub use bm25::Bm25Index;
pub use card::ToolCard;
pub use tokenizer::Tokenizer;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResult {
    pub tool: ToolCard,
    pub score: f32,
}

pub struct SearchEngine {
    index: Bm25Index,
}

impl SearchEngine {
    pub fn new(tools: Vec<ToolCard>) -> Self {
        Self {
            index: Bm25Index::from_cards(tools),
        }
    }

    pub fn search(&self, query: &str, limit: usize) -> Vec<SearchResult> {
        if limit == 0 {
            return Vec::new();
        }

        let mut scores = self.index.score_query(query);
        scores.retain(|&(_, score)| score > 0.0);
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        scores
            .into_iter()
            .take(limit)
            .map(|(doc_id, score)| SearchResult {
                tool: self.index.card(doc_id).clone(),
                score: score as f32,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_tools() -> Vec<ToolCard> {
        vec![
            ToolCard::new(
                "create_issue",
                "github",
                "Create a new issue on GitHub repository",
                vec!["title".to_string(), "body".to_string(), "repo".to_string()],
            ),
            ToolCard::new(
                "close_issue",
                "github",
                "Close an existing issue on GitHub repository",
                vec!["issue_number".to_string(), "repo".to_string()],
            ),
            ToolCard::new(
                "send_message",
                "slack",
                "Post a message to a Slack channel",
                vec!["channel".to_string(), "text".to_string()],
            ),
            ToolCard::new(
                "query_db",
                "postgres",
                "Execute a SQL query against PostgreSQL database",
                vec!["sql".to_string()],
            ),
        ]
    }

    #[test]
    fn test_search_engine_empty_tools() {
        let engine = SearchEngine::new(Vec::new());
        assert!(engine.search("github issue", 5).is_empty());
    }

    #[test]
    fn test_search_engine_zero_limit() {
        let engine = SearchEngine::new(sample_tools());
        assert!(engine.search("github issue", 0).is_empty());
    }

    #[test]
    fn test_search_engine_ranking() {
        let engine = SearchEngine::new(sample_tools());
        let results = engine.search("github issue", 5);

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].tool.server, "github");
        assert_eq!(results[1].tool.server, "github");
        assert!(results[0].score >= results[1].score);
    }

    #[test]
    fn test_search_engine_specific_tool() {
        let engine = SearchEngine::new(sample_tools());
        let results = engine.search("slack message channel", 2);

        assert!(!results.is_empty());
        assert_eq!(results[0].tool.name, "send_message");
        assert_eq!(results[0].tool.server, "slack");
    }

    #[test]
    fn test_search_engine_limit() {
        let engine = SearchEngine::new(sample_tools());
        let results = engine.search("github", 1);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_search_engine_no_matches() {
        let engine = SearchEngine::new(sample_tools());
        assert!(engine.search("kubernetes pod", 5).is_empty());
    }
}
