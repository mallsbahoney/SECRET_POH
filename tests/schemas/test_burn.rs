#[test]
fn test_burn_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::burn::execute(&params, &mut state).is_ok());
}
