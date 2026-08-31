//! `doro-policy` implements tool rules (allow, ask, deny) and profile resolution.

use std::collections::HashMap;

use doro_core::search::ToolCard;
use serde::{Deserialize, Serialize};
use wildmatch::WildMatch;

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

    #[error("Failed to parse profile config: {0}")]
    Parse(#[from] serde_json::Error),

    #[error("Unknown profile: {0}")]
    UnknownProfile(String),

    #[error("No pending approval with id {0}")]
    ApprovalNotFound(u64),
}

pub type Result<T> = std::result::Result<T, PolicyError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRule {
    pub pattern: String,
    pub action: PolicyAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    #[serde(rename = "default")]
    pub default_action: PolicyAction,
    #[serde(default)]
    pub rules: Vec<PolicyRule>,
}

impl Profile {
    pub fn resolve(&self, qualified_tool: &str) -> PolicyAction {
        for rule in &self.rules {
            if WildMatch::new(&rule.pattern).matches(qualified_tool) {
                return rule.action;
            }
        }
        self.default_action
    }

    pub fn visible_tools(&self, tools: Vec<ToolCard>) -> Vec<ToolCard> {
        tools
            .into_iter()
            .filter(|tool| self.resolve(&tool.qualified_name()) != PolicyAction::Deny)
            .collect()
    }
}

#[derive(Debug, Clone, Default)]
pub struct ProfileSet(HashMap<String, Profile>);

#[derive(Deserialize)]
struct ConfigFile {
    #[serde(default)]
    profiles: HashMap<String, Profile>,
}

impl ProfileSet {
    pub fn from_config_json(json: &str) -> Result<Self> {
        let config: ConfigFile = serde_json::from_str(json)?;
        Ok(Self(config.profiles))
    }

    pub fn get(&self, name: &str) -> Result<&Profile> {
        self.0
            .get(name)
            .ok_or_else(|| PolicyError::UnknownProfile(name.to_string()))
    }

    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.0.keys().map(String::as_str)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingApproval {
    pub id: u64,
    pub tool: String,
}

#[derive(Debug, Default)]
pub struct ApprovalQueue {
    next_id: u64,
    pending: Vec<PendingApproval>,
}

impl ApprovalQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn submit(&mut self, tool: impl Into<String>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.pending.push(PendingApproval {
            id,
            tool: tool.into(),
        });
        id
    }

    pub fn list(&self) -> &[PendingApproval] {
        &self.pending
    }

    pub fn approve(&mut self, id: u64) -> Result<PendingApproval> {
        self.take(id)
    }

    pub fn deny(&mut self, id: u64) -> Result<PendingApproval> {
        self.take(id)
    }

    fn take(&mut self, id: u64) -> Result<PendingApproval> {
        let index = self
            .pending
            .iter()
            .position(|p| p.id == id)
            .ok_or(PolicyError::ApprovalNotFound(id))?;
        Ok(self.pending.remove(index))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_profile() -> Profile {
        Profile {
            default_action: PolicyAction::Allow,
            rules: vec![
                PolicyRule {
                    pattern: "github.create_issue".to_string(),
                    action: PolicyAction::Deny,
                },
                PolicyRule {
                    pattern: "github.*".to_string(),
                    action: PolicyAction::Ask,
                },
                PolicyRule {
                    pattern: "slack.*".to_string(),
                    action: PolicyAction::Deny,
                },
            ],
        }
    }

    #[test]
    fn test_policy_action_serialization() {
        let action = PolicyAction::Allow;
        let json = serde_json::to_string(&action).unwrap();
        assert_eq!(json, "\"allow\"");

        let deserialized: PolicyAction = serde_json::from_str("\"deny\"").unwrap();
        assert_eq!(deserialized, PolicyAction::Deny);
    }

    #[test]
    fn test_policy_error_violation_display() {
        let err = PolicyError::Violation("blocked".to_string());
        assert_eq!(err.to_string(), "Policy violation: blocked");
    }

    #[test]
    fn test_resolve_exact_match_beats_wildcard() {
        let profile = sample_profile();
        assert_eq!(profile.resolve("github.create_issue"), PolicyAction::Deny);
    }

    #[test]
    fn test_resolve_wildcard_match() {
        let profile = sample_profile();
        assert_eq!(profile.resolve("github.close_issue"), PolicyAction::Ask);
        assert_eq!(profile.resolve("slack.send_message"), PolicyAction::Deny);
    }

    #[test]
    fn test_resolve_falls_back_to_default() {
        let profile = sample_profile();
        assert_eq!(profile.resolve("postgres.query_db"), PolicyAction::Allow);
    }

    #[test]
    fn test_visible_tools_hides_only_denied() {
        let profile = sample_profile();
        let tools = vec![
            ToolCard::new("create_issue", "github", "desc", Vec::new()),
            ToolCard::new("close_issue", "github", "desc", Vec::new()),
            ToolCard::new("send_message", "slack", "desc", Vec::new()),
            ToolCard::new("query_db", "postgres", "desc", Vec::new()),
        ];

        let visible = profile.visible_tools(tools);
        let names: Vec<&str> = visible.iter().map(|t| t.name.as_str()).collect();

        assert_eq!(names, vec!["close_issue", "query_db"]);
    }

    #[test]
    fn test_profile_set_from_config_json() {
        let json = r#"{
            "servers": { "github": { "command": "npx" } },
            "profiles": {
                "ci": { "default": "deny", "rules": [
                    { "pattern": "github.get_issue", "action": "allow" }
                ]},
                "default": { "default": "allow", "rules": [] }
            }
        }"#;

        let profiles = ProfileSet::from_config_json(json).unwrap();
        let mut names: Vec<&str> = profiles.names().collect();
        names.sort();
        assert_eq!(names, vec!["ci", "default"]);

        let ci = profiles.get("ci").unwrap();
        assert_eq!(ci.default_action, PolicyAction::Deny);
        assert_eq!(ci.resolve("github.get_issue"), PolicyAction::Allow);
        assert_eq!(ci.resolve("github.delete_repo"), PolicyAction::Deny);
    }

    #[test]
    fn test_profile_set_unknown_profile() {
        let profiles = ProfileSet::from_config_json(r#"{"profiles": {}}"#).unwrap();
        assert!(matches!(
            profiles.get("missing"),
            Err(PolicyError::UnknownProfile(_))
        ));
    }

    #[test]
    fn test_approval_queue_submit_and_list() {
        let mut queue = ApprovalQueue::new();
        let id = queue.submit("slack.delete_channel");
        assert_eq!(queue.list().len(), 1);
        assert_eq!(queue.list()[0].id, id);
        assert_eq!(queue.list()[0].tool, "slack.delete_channel");
    }

    #[test]
    fn test_approval_queue_approve_removes_entry() {
        let mut queue = ApprovalQueue::new();
        let id = queue.submit("slack.delete_channel");
        let approved = queue.approve(id).unwrap();
        assert_eq!(approved.tool, "slack.delete_channel");
        assert!(queue.list().is_empty());
    }

    #[test]
    fn test_approval_queue_unknown_id() {
        let mut queue = ApprovalQueue::new();
        assert!(matches!(
            queue.approve(42),
            Err(PolicyError::ApprovalNotFound(42))
        ));
    }
}
