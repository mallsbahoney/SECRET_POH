#[test]
fn test_callapp_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::callapp::execute(&params, &mut state).is_ok());
}
