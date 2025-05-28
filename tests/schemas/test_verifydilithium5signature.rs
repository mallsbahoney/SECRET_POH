#[test]
fn test_verifydilithium5signature_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::verifydilithium5signature::execute(&params, &mut state).is_ok());
}
