
pub fn execute(params: &SchemaParams, state: &mut ChainState) -> Result<(), SchemaError> {
    let group_id = params.get_str("group_id")?;
    let access_map = params.get_map("owners")?;
    for (owner, access_level) in access_map {
        state.grant_access_level(&group_id, &owner, &access_level)?;
    }
    Ok(())
}
