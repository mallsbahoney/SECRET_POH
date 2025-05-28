#[test]
fn test_createdilithium3signature_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::createdilithium3signature::execute(&params, &mut state).is_ok());
}
