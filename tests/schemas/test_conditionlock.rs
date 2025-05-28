#[test]
fn test_conditionlock_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::conditionlock::execute(&params, &mut state).is_ok());
}
