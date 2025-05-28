use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref RAM_SNAPSHOT: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
}

pub fn write_to_snapshot(key: &str, value: &str) {
    RAM_SNAPSHOT.lock().unwrap().insert(key.to_string(), value.to_string());
}

pub fn read_from_snapshot(key: &str) -> Option<String> {
    RAM_SNAPSHOT.lock().unwrap().get(key).cloned()
}
