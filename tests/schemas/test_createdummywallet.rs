#[test]
fn test_createdummywallet_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::createdummywallet::execute(&params, &mut state).is_ok());
}
