#[test]
fn test_mint_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::mint::execute(&params, &mut state).is_ok());
}
