#[test]
fn test_createnativesuiwalletbalance_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::createnativesuiwalletbalance::execute(&params, &mut state).is_ok());
}
