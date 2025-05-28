#[test]
fn test_triggernativebtcwalletbalance_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::triggernativebtcwalletbalance::execute(&params, &mut state).is_ok());
}
