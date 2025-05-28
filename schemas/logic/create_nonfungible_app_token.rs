
pub fn execute(params: &SchemaParams, state: &mut ChainState) -> Result<(), SchemaError> {
    let name = params.get_str("name")?;
    let metadata_uri = params.get_str("metadata_uri")?;
    let owner = params.get_sender();

    let app_id = state.create_app(name.clone(), owner)?;
    state.set_app_kv(&app_id, "uri", metadata_uri)?;
    state.set_app_kv(&app_id, "nft", "true")?;
    state.set_balance(&owner, &app_id, 1)?;

    Ok(())
}
