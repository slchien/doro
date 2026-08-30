#[derive(Debug, thiserror::Error)]
pub enum VaultError {
    #[error("Secret not found: {0}")]
    NotFound(String),

    #[error("Keyring error: {0}")]
    Keyring(String),
}

pub type Result<T> = std::result::Result<T, VaultError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vault_error_display() {
        let err = VaultError::NotFound("api_key".to_string());
        assert_eq!(err.to_string(), "Secret not found: api_key");
    }
}
