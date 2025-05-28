#[test]
fn test_verifyexternaldata_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::verifyexternaldata::execute(&params, &mut state).is_ok());
}
