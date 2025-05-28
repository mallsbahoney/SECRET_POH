
pub fn execute(params: &SchemaParams, state: &mut ChainState) -> Result<(), SchemaError> {
    let app_id = params.get_str("app_id")?;
    state.create_lp_pot("app", &[&app_id])?;
    Ok(())
}
