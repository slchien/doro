use doro_core::search::{SearchEngine, ToolCard};
use doro_policy::{PolicyAction, PolicyRule, Profile};

fn sample_tools() -> Vec<ToolCard> {
    vec![
        ToolCard::new(
            "create_issue",
            "github",
            "Create a new issue on GitHub repository",
            vec!["title".to_string()],
        ),
        ToolCard::new(
            "close_issue",
            "github",
            "Close an existing issue on GitHub repository",
            vec!["issue_number".to_string()],
        ),
        ToolCard::new(
            "send_message",
            "slack",
            "Post a message to a Slack channel",
            vec!["channel".to_string()],
        ),
        ToolCard::new(
            "delete_channel",
            "slack",
            "Permanently delete a Slack channel",
            vec!["channel".to_string()],
        ),
        ToolCard::new(
            "query_db",
            "postgres",
            "Execute a SQL query against PostgreSQL database",
            vec!["sql".to_string()],
        ),
    ]
}

fn ci_profile() -> Profile {
    Profile {
        default_action: PolicyAction::Deny,
        rules: vec![
            PolicyRule {
                pattern: "github.create_issue".to_string(),
                action: PolicyAction::Allow,
            },
            PolicyRule {
                pattern: "postgres.query_db".to_string(),
                action: PolicyAction::Allow,
            },
        ],
    }
}

#[test]
fn ci_profile_resolves_only_the_allowlist() {
    let profile = ci_profile();

    assert_eq!(profile.resolve("github.create_issue"), PolicyAction::Allow);
    assert_eq!(profile.resolve("postgres.query_db"), PolicyAction::Allow);

    assert_eq!(profile.resolve("github.close_issue"), PolicyAction::Deny);
    assert_eq!(profile.resolve("slack.send_message"), PolicyAction::Deny);
    assert_eq!(profile.resolve("slack.delete_channel"), PolicyAction::Deny);
}

#[test]
fn ci_profile_can_only_see_its_allowed_tools() {
    let profile = ci_profile();
    let visible = profile.visible_tools(sample_tools());

    assert_eq!(visible.len(), 2);
    let engine = SearchEngine::new(visible);

    assert!(engine.search("close_issue", 5).is_empty());
    assert!(engine.search("delete_channel", 5).is_empty());
    assert!(engine.search("slack channel message", 5).is_empty());

    let results = engine.search("github issue", 5);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].tool.name, "create_issue");

    let results = engine.search("postgres sql query", 5);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].tool.name, "query_db");
}
