// [INJECTED - STORAGE NODE LOGIC]
pub fn log_to_storage(raw_data: &str) -> Result<(), String> {
    println!("[Storage Node] Data received and logged: {}", raw_data);
    Ok(())
}