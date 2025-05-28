#[test]
fn test_parallellink_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::parallellink::execute(&params, &mut state).is_ok());
}
