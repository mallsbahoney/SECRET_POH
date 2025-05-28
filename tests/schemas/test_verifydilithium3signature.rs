#[test]
fn test_verifydilithium3signature_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::verifydilithium3signature::execute(&params, &mut state).is_ok());
}
