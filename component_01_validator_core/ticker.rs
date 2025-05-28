
use std::time::{SystemTime, UNIX_EPOCH};
use sha3::{Digest, Sha3_256};

pub fn generate_validator_tick(codebase: &[u8], sys_power: &[u8]) -> (u128, String) {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let mut hasher = Sha3_256::new();
    hasher.update(codebase);
    hasher.update(sys_power);
    hasher.update(timestamp.to_le_bytes());
    let hash = hex::encode(hasher.finalize());
    (timestamp, hash)
}
