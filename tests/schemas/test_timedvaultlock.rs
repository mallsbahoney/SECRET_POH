#[test]
fn test_timedvaultlock_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::timedvaultlock::execute(&params, &mut state).is_ok());
}
