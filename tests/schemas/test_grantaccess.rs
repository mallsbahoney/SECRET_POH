#[test]
fn test_grantaccess_execution() {
    let params = SchemaParams::default();
    let mut state = ChainState::new();
    assert!(super::logic::grantaccess::execute(&params, &mut state).is_ok());
}
