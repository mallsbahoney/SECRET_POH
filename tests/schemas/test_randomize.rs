#[test]
fn test_randomize_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::randomize::execute(&params, &mut state).is_ok());
}
