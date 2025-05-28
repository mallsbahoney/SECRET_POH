#[test]
fn test_time_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::time::execute(&params, &mut state).is_ok());
}
