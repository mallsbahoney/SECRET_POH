#[test]
fn test_unlockhtlc_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::unlockhtlc::execute(&params, &mut state).is_ok());
}
