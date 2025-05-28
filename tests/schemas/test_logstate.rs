#[test]
fn test_logstate_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::logstate::execute(&params, &mut state).is_ok());
}
