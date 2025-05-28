
pub fn execute(params: &SchemaParams, state: &mut ChainState) -> Result<(), SchemaError> {
    let token_a = params.get_str("token_a")?;
    let token_b = params.get_str("token_b")?;
    let pool_id = state.create_lp_pot("token", &[&token_a, &token_b])?;
    Ok(())
}
