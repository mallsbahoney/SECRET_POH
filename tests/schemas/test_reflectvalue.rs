#[test]
fn test_reflectvalue_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::reflectvalue::execute(&params, &mut state).is_ok());
}
