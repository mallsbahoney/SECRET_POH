
// Encrypted schema input support using hash commitment scheme
use sha3::{Digest, Sha3_256};

pub fn verify_commitment(input: &str, salt: &str, expected_hash: &str) -> bool {
    let mut hasher = Sha3_256::new();
    hasher.update(format!("{}{}", input, salt));
    let result = format!("{:x}", hasher.finalize());
    result == expected_hash
}
