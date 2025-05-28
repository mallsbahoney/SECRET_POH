#[test]
fn test_conditionvaultlock_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::conditionvaultlock::execute(&params, &mut state).is_ok());
}
