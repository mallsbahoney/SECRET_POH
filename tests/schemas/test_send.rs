#[test]
fn test_send_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::send::execute(&params, &mut state).is_ok());
}
