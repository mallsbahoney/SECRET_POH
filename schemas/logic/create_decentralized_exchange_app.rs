
pub fn execute(params: &SchemaParams, state: &mut ChainState) -> Result<(), SchemaError> {
    let creator = params.get_sender();
    let dex_id = state.create_app("DEX", creator)?;
    state.set_app_kv(&dex_id, "type", "decentralized")?;
    Ok(())
}
