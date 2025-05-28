#[test]
fn test_swap_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::swap::execute(&params, &mut state).is_ok());
}
