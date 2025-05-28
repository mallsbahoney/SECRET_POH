#[test]
fn test_createnativeadawalletbalance_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::createnativeadawalletbalance::execute(&params, &mut state).is_ok());
}
