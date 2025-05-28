
pub fn execute(params: &SchemaParams, state: &mut ChainState) -> Result<(), SchemaError> {
    let key = params.get_str("key")?;
    let value = state.read_key(&key)?;
    state.emit("read_value", &value)?;
    Ok(())
}
