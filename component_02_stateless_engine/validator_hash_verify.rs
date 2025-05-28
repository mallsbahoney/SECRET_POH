
// Instruction-level validator hash check
use sha3::{Digest, Sha3_256};
use std::fs;

pub fn verify_validator_code(path: &str, expected_hash: &str) -> bool {
    let code = fs::read_to_string(path).unwrap_or_default();
    let mut hasher = Sha3_256::new();
    hasher.update(code);
    let result = format!("{:x}", hasher.finalize());
    result == expected_hash
}
