// Mesh registry: validator heartbeat + global sync
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct ValidatorMeta {
    pub id: String,
    pub region: String,
    pub system_hash: String,
    pub last_heartbeat: u128,
}

lazy_static::lazy_static! {
    pub static ref MESH_REGISTRY: Arc<Mutex<HashMap<String, ValidatorMeta>>> = Arc::new(Mutex::new(HashMap::new()));
}

pub fn broadcast_heartbeat(id: &str, region: &str, system_hash: &str) {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let meta = ValidatorMeta {
        id: id.to_string(),
        region: region.to_string(),
        system_hash: system_hash.to_string(),
        last_heartbeat: now,
    };
    let mut registry = MESH_REGISTRY.lock().unwrap();
    registry.insert(id.to_string(), meta);
    println!("[HEARTBEAT] {} at {}", id, now);
}

pub fn get_all_online() -> Vec<ValidatorMeta> {
    let registry = MESH_REGISTRY.lock().unwrap();
    registry.values().cloned().collect()
}
