#[test]
fn test_emitevent_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::emitevent::execute(&params, &mut state).is_ok());
}
