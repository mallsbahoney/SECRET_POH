#[test]
fn test_vault_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::vault::execute(&params, &mut state).is_ok());
}
