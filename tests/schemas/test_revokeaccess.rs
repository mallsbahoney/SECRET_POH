#[test]
fn test_revokeaccess_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::revokeaccess::execute(&params, &mut state).is_ok());
}
