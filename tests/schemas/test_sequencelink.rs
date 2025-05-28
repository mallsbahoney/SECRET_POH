#[test]
fn test_sequencelink_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::sequencelink::execute(&params, &mut state).is_ok());
}
