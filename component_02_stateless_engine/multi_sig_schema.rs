
// Threshold multisig schema (e.g., 2-of-3 approval)
use std::collections::HashSet;

pub struct MultiSigSchema {
    pub threshold: usize,
    pub approved: HashSet<String>,
}

impl MultiSigSchema {
    pub fn new(threshold: usize) -> Self {
        Self {
            threshold,
            approved: HashSet::new(),
        }
    }

    pub fn approve(&mut self, signer: &str) {
        self.approved.insert(signer.to_string());
    }

    pub fn is_approved(&self) -> bool {
        self.approved.len() >= self.threshold
    }
}
