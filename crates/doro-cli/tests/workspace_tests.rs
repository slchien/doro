#[test]
fn test_workspace_crate_linking() {
    assert_eq!(doro_core::VERSION, "0.1.0");

    let action = doro_policy::PolicyAction::Allow;
    assert_eq!(action, doro_policy::PolicyAction::Allow);

    let err = doro_vault::VaultError::NotFound("test".to_string());
    assert_eq!(err.to_string(), "Secret not found: test");
}
