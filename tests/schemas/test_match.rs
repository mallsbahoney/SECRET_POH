#[test]
fn test_match_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::match::execute(&params, &mut state).is_ok());
}
