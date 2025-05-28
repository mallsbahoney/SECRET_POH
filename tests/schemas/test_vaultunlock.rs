#[test]
fn test_vaultunlock_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::vaultunlock::execute(&params, &mut state).is_ok());
}
