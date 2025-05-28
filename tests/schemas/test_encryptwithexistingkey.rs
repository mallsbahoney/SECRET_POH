#[test]
fn test_encryptwithexistingkey_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::encryptwithexistingkey::execute(&params, &mut state).is_ok());
}
