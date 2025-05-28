#[test]
fn test_triggercustomnativewalletbalance_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::triggercustomnativewalletbalance::execute(&params, &mut state).is_ok());
}
