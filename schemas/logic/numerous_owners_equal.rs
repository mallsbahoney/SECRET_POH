
pub fn execute(params: &SchemaParams, state: &mut ChainState) -> Result<(), SchemaError> {
    let group_id = params.get_str("group_id")?;
    let owners = params.get_list("owners")?;
    for owner in owners {
        state.grant_access(&group_id, &owner)?;
    }
    Ok(())
}
