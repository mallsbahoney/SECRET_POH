
pub fn execute(params: &SchemaParams, state: &mut ChainState) -> Result<(), SchemaError> {
    let condition_met = params.get_bool("condition")?;
    if condition_met {
        state.cancel_next_in_sequence()?;
    }
    Ok(())
}
