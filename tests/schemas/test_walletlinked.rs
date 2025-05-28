#[test]
fn test_walletlinked_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::walletlinked::execute(&params, &mut state).is_ok());
}
