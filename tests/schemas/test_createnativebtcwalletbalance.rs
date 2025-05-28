#[test]
fn test_createnativebtcwalletbalance_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::createnativebtcwalletbalance::execute(&params, &mut state).is_ok());
}
