pub struct Tokenizer;

impl Tokenizer {
    pub fn tokenize(text: &str) -> Vec<String> {
        text.to_lowercase()
            .split_whitespace()
            .map(|token| token.trim_matches(|c: char| !c.is_alphanumeric()))
            .filter(|token| token.len() > 1)
            .map(|s| s.to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_basic() {
        let text = "Hello World";
        let tokens = Tokenizer::tokenize(text);
        assert_eq!(tokens, vec!["hello", "world"]);
    }

    #[test]
    fn test_tokenize_empty() {
        let text = "";
        let tokens = Tokenizer::tokenize(text);
        assert!(tokens.is_empty());
    }

    #[test]
    fn test_tokenize_whitespace() {
        let text = "  hello   world  ";
        let tokens = Tokenizer::tokenize(text);
        assert_eq!(tokens, vec!["hello", "world"]);
    }

    #[test]
    fn test_tokenize_mixed_case() {
        let text = "HeLlO wOrLd";
        let tokens = Tokenizer::tokenize(text);
        assert_eq!(tokens, vec!["hello", "world"]);
    }

    #[test]
    fn test_tokenize_with_punctuation() {
        let text = "Hello, world!";
        let tokens = Tokenizer::tokenize(text);
        assert_eq!(tokens, vec!["hello", "world"]);
    }

    #[test]
    fn test_tokenize_numbers() {
        let text = "123 456";
        let tokens = Tokenizer::tokenize(text);
        assert_eq!(tokens, vec!["123", "456"]);
    }

    #[test]
    fn test_tokenize_hyphenated() {
        let text = "state-of-the-art";
        let tokens = Tokenizer::tokenize(text);
        assert_eq!(tokens, vec!["state-of-the-art"]);
    }
}
