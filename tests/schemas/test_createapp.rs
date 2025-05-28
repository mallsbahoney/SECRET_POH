#[test]
fn test_createapp_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::createapp::execute(&params, &mut state).is_ok());
}
