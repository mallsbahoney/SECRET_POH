#[test]
fn test_createappbalance_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::createappbalance::execute(&params, &mut state).is_ok());
}
