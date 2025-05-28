#[test]
fn test_triggernativeadawalletbalance_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::triggernativeadawalletbalance::execute(&params, &mut state).is_ok());
}
