#[test]
fn test_compute_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::compute::execute(&params, &mut state).is_ok());
}
