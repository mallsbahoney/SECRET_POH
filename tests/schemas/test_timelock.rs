#[test]
fn test_timelock_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::timelock::execute(&params, &mut state).is_ok());
}
