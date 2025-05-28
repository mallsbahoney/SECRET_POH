#[test]
fn test_createdilithium5signature_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::createdilithium5signature::execute(&params, &mut state).is_ok());
}
