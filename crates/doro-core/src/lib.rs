pub mod search;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, thiserror::Error)]
pub enum DoroError {
    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid configuration: {0}")]
    Config(String),
}

pub type Result<T> = std::result::Result<T, DoroError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_present() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_error_formatting() {
        let err = DoroError::NotFound("test_resource".to_string());
        assert_eq!(err.to_string(), "Not found: test_resource");
    }
}
