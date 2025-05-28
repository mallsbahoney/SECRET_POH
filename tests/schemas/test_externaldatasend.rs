#[test]
fn test_externaldatasend_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::externaldatasend::execute(&params, &mut state).is_ok());
}
