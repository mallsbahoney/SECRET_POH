#[test]
fn test_condition_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::condition::execute(&params, &mut state).is_ok());
}
