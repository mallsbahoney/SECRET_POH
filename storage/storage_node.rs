// Storage node: CID + state log sync
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Write};
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct StorageEntry {
    pub cid: String,
    pub content: String,
    pub timestamp: u128,
}

pub struct StorageNode {
    pub log: HashMap<String, StorageEntry>,
}

impl StorageNode {
    pub fn new() -> Self {
        Self {
            log: HashMap::new(),
        }
    }

    pub fn backup(&mut self, cid: &str, content: &str) {
        let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
        let entry = StorageEntry {
            cid: cid.to_string(),
            content: content.to_string(),
            timestamp,
        };
        self.log.insert(cid.to_string(), entry.clone());
        let mut file = OpenOptions::new().create(true).append(true).open("storage_log.txt").unwrap();
        writeln!(file, "[{}] CID: {} => {}", timestamp, cid, content).unwrap();
        println!("[BACKUP] {}", cid);
    }
}
