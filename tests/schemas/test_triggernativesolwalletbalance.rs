#[test]
fn test_triggernativesolwalletbalance_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::triggernativesolwalletbalance::execute(&params, &mut state).is_ok());
}
