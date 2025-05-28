#[test]
fn test_sendmessage_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::sendmessage::execute(&params, &mut state).is_ok());
}
