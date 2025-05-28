
pub fn execute(params: &SchemaParams, state: &mut ChainState) -> Result<(), SchemaError> {
    let name = params.get_str("name")?;
    let symbol = params.get_str("symbol")?;
    let decimals = params.get_u8("decimals")?;
    let initial_supply = params.get_u64("initial_supply")?;
    let creator = params.get_sender();

    let app_id = state.create_app(name.clone(), creator)?;
    state.set_app_kv(&app_id, "symbol", symbol)?;
    state.set_app_kv(&app_id, "decimals", &decimals.to_string())?;
    state.set_balance(&creator, &app_id, initial_supply)?;

    Ok(())
}
