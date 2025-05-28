// Result returned by Processor Node
pub struct ProcessorExecutionResult {
    pub origin_wallet: String,
    pub processor_pubkey: String,
    pub schema_id: String,
    pub input_data: serde_json::Value,
    pub output_hash: String,
    pub processor_signature: String,
    pub reward_address: String,
    pub execution_time_ms: u64,
}