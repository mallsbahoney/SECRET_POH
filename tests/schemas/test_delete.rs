#[test]
fn test_delete_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::delete::execute(&params, &mut state).is_ok());
}
