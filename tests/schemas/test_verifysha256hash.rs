#[test]
fn test_verifysha256hash_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::verifysha256hash::execute(&params, &mut state).is_ok());
}
