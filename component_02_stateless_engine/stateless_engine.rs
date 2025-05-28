use sha3::{Digest, Sha3_256};
use chrono::Utc;
use std::collections::HashSet;

#[derive(Clone)]
pub struct StatelessValidator {
    seen_hashes: HashSet<String>,
}

impl StatelessValidator {
    pub fn new() -> Self {
        Self {
            seen_hashes: HashSet::new(),
        }
    }

    pub fn validate_tx(&mut self, sender: &str, nonce: u64, balance: u64) -> Result<String, &'static str> {
        let timestamp = Utc::now().timestamp_millis();
        let hash = Self::generate_hash(sender, nonce, balance, timestamp);

        if self.seen_hashes.contains(&hash) {
            return Err("Duplicate transaction detected");
        }

        self.seen_hashes.insert(hash.clone());
        Ok(hash)
    }

    fn generate_hash(sender: &str, nonce: u64, balance: u64, timestamp: i64) -> String {
        let mut hasher = Sha3_256::new();
        hasher.update(format!("{}{}{}{}", sender, nonce, balance, timestamp));
        format!("{:x}", hasher.finalize())
    }
}// [INJECTED - REMOVE 100ms LOOP]
pub fn process_live_tick(input_data: &str) -> Result<(), String> {
    println!("[Engine] Instant processing of input: {}", input_data);
    Ok(())
}