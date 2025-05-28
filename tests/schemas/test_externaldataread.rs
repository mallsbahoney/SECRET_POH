#[test]
fn test_externaldataread_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::externaldataread::execute(&params, &mut state).is_ok());
}
