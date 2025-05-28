#[test]
fn test_storekeyvalue_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::storekeyvalue::execute(&params, &mut state).is_ok());
}
