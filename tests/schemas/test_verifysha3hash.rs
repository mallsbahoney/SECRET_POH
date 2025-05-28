#[test]
fn test_verifysha3hash_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::verifysha3hash::execute(&params, &mut state).is_ok());
}
