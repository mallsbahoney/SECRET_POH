
pub fn execute(params: &SchemaParams, state: &mut ChainState) -> Result<(), SchemaError> {
    let dest_key = params.get_str("destination")?;
    let data = state.get_temp("copied_data")?;
    state.write_key(&dest_key, &data)?;
    Ok(())
}
