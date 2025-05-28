#[test]
fn test_getkeyvalue_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::getkeyvalue::execute(&params, &mut state).is_ok());
}
