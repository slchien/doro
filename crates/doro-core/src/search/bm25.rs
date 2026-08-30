use std::collections::HashMap;

use super::card::ToolCard;
use super::tokenizer::Tokenizer;

pub const DEFAULT_K1: f64 = 1.2;
pub const DEFAULT_B: f64 = 0.75;

#[derive(Debug, Clone)]
pub struct Bm25Index {
    cards: Vec<ToolCard>,
    doc_lengths: Vec<usize>,
    total_doc_length: usize,
    avg_doc_length: f64,
    inverted_index: HashMap<String, Vec<(usize, usize)>>,
    k1: f64,
    b: f64,
}

impl Default for Bm25Index {
    fn default() -> Self {
        Self::new()
    }
}

impl Bm25Index {
    pub fn new() -> Self {
        Self::with_params(DEFAULT_K1, DEFAULT_B)
    }

    pub fn with_params(k1: f64, b: f64) -> Self {
        Self {
            cards: Vec::new(),
            doc_lengths: Vec::new(),
            total_doc_length: 0,
            avg_doc_length: 0.0,
            inverted_index: HashMap::new(),
            k1,
            b,
        }
    }

    pub fn from_cards(cards: Vec<ToolCard>) -> Self {
        let mut index = Self::new();
        for card in cards {
            index.add(card);
        }
        index
    }

    pub fn add(&mut self, card: ToolCard) {
        let doc_id = self.cards.len();
        let tokens = Tokenizer::tokenize(&card.searchable_text());
        let doc_len = tokens.len();

        let mut term_counts: HashMap<String, usize> = HashMap::new();
        for token in tokens {
            *term_counts.entry(token).or_insert(0) += 1;
        }

        for (term, count) in term_counts {
            self.inverted_index
                .entry(term)
                .or_default()
                .push((doc_id, count));
        }

        self.cards.push(card);
        self.doc_lengths.push(doc_len);
        self.total_doc_length += doc_len;
        self.avg_doc_length = self.total_doc_length as f64 / self.cards.len() as f64;
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn card(&self, doc_id: usize) -> &ToolCard {
        &self.cards[doc_id]
    }

    /// Computes Robertson-Spärck Jones IDF with 1.0 added inside the logarithm
    /// to ensure IDF is non-negative even for terms appearing in all documents.
    fn idf(&self, doc_freq: usize) -> f64 {
        let n = self.cards.len() as f64;
        let df = doc_freq as f64;
        let numerator = n - df + 0.5;
        let denominator = df + 0.5;
        ((numerator / denominator) + 1.0).ln()
    }

    /// Scores every indexed document against `query`, returning `(doc_id, score)`
    /// pairs in doc-id order, unsorted and unfiltered (non-matching docs score 0.0).
    pub fn score_query(&self, query: &str) -> Vec<(usize, f64)> {
        if self.cards.is_empty() {
            return Vec::new();
        }

        let query_tokens = Tokenizer::tokenize(query);
        if query_tokens.is_empty() {
            return Vec::new();
        }

        let mut scores = vec![0.0f64; self.cards.len()];

        for term in &query_tokens {
            if let Some(postings) = self.inverted_index.get(term) {
                let idf_score = self.idf(postings.len());

                for &(doc_id, tf) in postings {
                    let doc_len = self.doc_lengths[doc_id] as f64;
                    let tf = tf as f64;

                    let length_norm = if self.avg_doc_length > 0.0 {
                        1.0 - self.b + self.b * (doc_len / self.avg_doc_length)
                    } else {
                        1.0
                    };

                    let term_score =
                        idf_score * (tf * (self.k1 + 1.0)) / (tf + self.k1 * length_norm);
                    scores[doc_id] += term_score;
                }
            }
        }

        scores.into_iter().enumerate().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_cards() -> Vec<ToolCard> {
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
    fn test_bm25_empty_index() {
        let index = Bm25Index::new();
        assert!(index.is_empty());
        assert_eq!(index.len(), 0);
        assert!(index.score_query("github issue").is_empty());
    }

    #[test]
    fn test_bm25_empty_query() {
        let index = Bm25Index::from_cards(sample_cards());
        assert!(index.score_query("").is_empty());
    }

    #[test]
    fn test_bm25_scores_matching_docs_only() {
        let index = Bm25Index::from_cards(sample_cards());
        let scores = index.score_query("github issue");
        assert_eq!(scores.len(), index.len());

        let matching: Vec<f64> = scores
            .iter()
            .filter(|(doc_id, _)| index.card(*doc_id).server == "github")
            .map(|(_, score)| *score)
            .collect();
        assert_eq!(matching.len(), 2);
        assert!(matching.iter().all(|&score| score > 0.0));

        let non_matching_all_zero = scores
            .iter()
            .filter(|(doc_id, _)| index.card(*doc_id).server != "github")
            .all(|(_, score)| *score == 0.0);
        assert!(non_matching_all_zero);
    }

    #[test]
    fn test_bm25_no_matches() {
        let index = Bm25Index::from_cards(sample_cards());
        let scores = index.score_query("kubernetes pod");
        assert!(scores.iter().all(|(_, score)| *score == 0.0));
    }

    #[test]
    fn test_bm25_matches_despite_sentence_punctuation() {
        let index = Bm25Index::from_cards(vec![ToolCard::new(
            "get_weather",
            "weather",
            "Query the current weather in a given city.",
            vec!["city".to_string()],
        )]);
        let scores = index.score_query("city");
        assert_eq!(scores.len(), 1);
        assert!(scores[0].1 > 0.0);
    }
}
