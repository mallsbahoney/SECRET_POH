#[test]
fn test_triggernativeethwalletbalance_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::triggernativeethwalletbalance::execute(&params, &mut state).is_ok());
}
