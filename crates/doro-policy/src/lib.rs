use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyAction {
    Allow,
    Ask,
    Deny,
}

#[derive(Debug, thiserror::Error)]
pub enum PolicyError {
    #[error("Policy violation: {0}")]
    Violation(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_action_serialization() {
        let action = PolicyAction::Allow;
        let json = serde_json::to_string(&action).unwrap();
        assert_eq!(json, "\"allow\"");

        let deserialized: PolicyAction = serde_json::from_str("\"deny\"").unwrap();
        assert_eq!(deserialized, PolicyAction::Deny);
    }
}
