
pub fn execute(params: &SchemaParams, state: &mut ChainState) -> Result<(), SchemaError> {
    let source_key = params.get_str("source")?;
    let data = state.read_key(&source_key)?;
    state.set_temp("copied_data", &data)?;
    Ok(())
}
