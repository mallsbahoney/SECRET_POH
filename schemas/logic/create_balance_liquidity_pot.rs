
pub fn execute(params: &SchemaParams, state: &mut ChainState) -> Result<(), SchemaError> {
    let balance_key = params.get_str("balance_id")?;
    state.create_lp_pot("balance", &[&balance_key])?;
    Ok(())
}
