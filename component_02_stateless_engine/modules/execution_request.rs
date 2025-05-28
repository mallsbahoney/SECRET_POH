// Wallet-to-Processor Execution Schema
pub struct ExecutionRequest {
    pub origin_wallet: String,
    pub schema_id: String,
    pub input_data: serde_json::Value,
    pub execution_mode: String,
    pub required_complexity: u8,
    pub txn_size_bytes: u32,
    pub latency_preference: String,
    pub processor_hint: Option<String>,
}