#[test]
fn test_update_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::update::execute(&params, &mut state).is_ok());
}
