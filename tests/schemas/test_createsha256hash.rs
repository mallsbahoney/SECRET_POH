#[test]
fn test_createsha256hash_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::createsha256hash::execute(&params, &mut state).is_ok());
}
