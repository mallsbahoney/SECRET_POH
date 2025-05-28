#[test]
fn test_encryptwithnewkey_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::encryptwithnewkey::execute(&params, &mut state).is_ok());
}
