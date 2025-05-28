#[test]
fn test_batchsend_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::batchsend::execute(&params, &mut state).is_ok());
}
