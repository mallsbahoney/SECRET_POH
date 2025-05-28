// Real state: balances and tick tracking
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

lazy_static::lazy_static! {
    pub static ref BALANCES: Arc<Mutex<HashMap<String, u64>>> = Arc::new(Mutex::new(HashMap::new()));
    pub static ref CURRENT_TICK: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));
}

pub fn update_balance(wallet: &str, amount: i64) -> bool {
    let mut balances = BALANCES.lock().unwrap();
    let current = *balances.get(wallet).unwrap_or(&0);
    if amount.is_negative() && current < amount.unsigned_abs() {
        return false;
    }
    let new_balance = (current as i64 + amount) as u64;
    balances.insert(wallet.to_string(), new_balance);
    true
}

pub fn get_balance(wallet: &str) -> u64 {
    let balances = BALANCES.lock().unwrap();
    *balances.get(wallet).unwrap_or(&0)
}

pub fn increment_tick() {
    let mut t = CURRENT_TICK.lock().unwrap();
    *t += 1;
}

pub fn get_tick() -> u64 {
    let t = CURRENT_TICK.lock().unwrap();
    *t
}
