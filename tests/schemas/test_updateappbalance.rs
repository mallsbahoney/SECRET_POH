#[test]
fn test_updateappbalance_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::updateappbalance::execute(&params, &mut state).is_ok());
}
