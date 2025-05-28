#[test]
fn test_declareschema_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::declareschema::execute(&params, &mut state).is_ok());
}
