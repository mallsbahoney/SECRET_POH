#[test]
fn test_createnativesolwalletbalance_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::createnativesolwalletbalance::execute(&params, &mut state).is_ok());
}
