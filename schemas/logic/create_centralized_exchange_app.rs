
pub fn execute(params: &SchemaParams, state: &mut ChainState) -> Result<(), SchemaError> {
    let admin = params.get_sender();
    let cex_id = state.create_app("CEX", admin)?;
    state.set_app_kv(&cex_id, "type", "centralized")?;
    Ok(())
}
