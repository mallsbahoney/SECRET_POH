
use std::sync::Mutex;
use once_cell::sync::Lazy;
use sha3::{Digest, Sha3_256};
use serde_json::Value;
use std::fs::{OpenOptions, File};
use std::io::{Write, BufReader, BufRead, Read};

static LAST_OUTPUT: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));
static BALANCE_FILE: &str = "user_balances.json";

pub fn execute_processor_task(input_json: &str) -> Result<String, String> {
    let input: Value = serde_json::from_str(input_json).map_err(|_| "Invalid JSON")?;
    let task_type = input["task"].as_str().unwrap_or("noop");

    let result = match task_type {
        "hash" => {
            let data = input["data"].as_str().unwrap_or("").as_bytes();
            let mut hasher = Sha3_256::new();
            hasher.update(data);
            let hash = hex::encode(hasher.finalize());
            json!({ "type": "hash", "input": data, "hash": hash })
        },
        "compute" => {
            let n = input["value"].as_i64().unwrap_or(0);
            let factorial = (1..=n).product::<i64>();
            json!({ "type": "compute", "input": n, "factorial": factorial })
        },
        "mint" => {
            let user = input["wallet"].as_str().unwrap_or("default_user");
            let amount = input["amount"].as_u64().unwrap_or(0);
            update_balance(user, amount as i64);
            let txn = format!("mint|{}|{}|system", user, amount);
            broadcast_to_chain(&txn);
            json!({ "type": "mint", "status": "success", "user": user, "amount": amount })
        },
        "send" => {
            let from = input["from"].as_str().unwrap_or("sender");
            let to = input["to"].as_str().unwrap_or("receiver");
            let amount = input["amount"].as_u64().unwrap_or(0);
            let success = transfer_balance(from, to, amount as i64);
            if success {
                let txn = format!("send|{}|{}|{}", from, to, amount);
                broadcast_to_chain(&txn);
                json!({ "type": "send", "status": "success", "from": from, "to": to, "amount": amount })
            } else {
                json!({ "type": "send", "status": "failed", "reason": "insufficient_balance", "from": from, "amount": amount })
            }
        },
        _ => json!({ "type": "noop", "message": "No recognized task" })
    };

    let output_str = result.to_string();
    let mut last = LAST_OUTPUT.lock().unwrap();
    *last = Some(output_str.clone());
    Ok(output_str)
}

fn update_balance(user: &str, delta: i64) {
    let mut balances = load_balances();
    let entry = balances.entry(user.to_string()).or_insert(0);
    if delta.is_positive() {
        *entry += delta as u64;
    }
    save_balances(&balances);
}

fn transfer_balance(from: &str, to: &str, amount: i64) -> bool {
    let mut balances = load_balances();
    let from_balance = balances.get(from).copied().unwrap_or(0);
    if from_balance < amount as u64 {
        return false;
    }
    *balances.entry(from.to_string()).or_insert(0) -= amount as u64;
    *balances.entry(to.to_string()).or_insert(0) += amount as u64;
    save_balances(&balances);
    true
}

fn save_balances(balances: &std::collections::HashMap<String, u64>) {
    if let Ok(mut file) = File::create(BALANCE_FILE) {
        let json = serde_json::to_string_pretty(&balances).unwrap();
        let _ = file.write_all(json.as_bytes());
    }
    verify_consistency();
}

fn load_balances() -> std::collections::HashMap<String, u64> {
    if let Ok(file) = File::open(BALANCE_FILE) {
        if let Ok(existing) = serde_json::from_reader::<_, serde_json::Map<String, Value>>(file) {
            let mut map = std::collections::HashMap::new();
            for (k, v) in existing {
                map.insert(k, v.as_u64().unwrap_or(0));
            }
            return map;
        }
    }
    std::collections::HashMap::new()
}

fn verify_consistency() {
    if let Ok(mut file) = File::open(BALANCE_FILE) {
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        let mut hasher = Sha3_256::new();
        hasher.update(&buf);
        let hash = hex::encode(hasher.finalize());
        broadcast_to_chain(&format!("balance_hash|{}", hash));
    }
}

fn broadcast_to_chain(txn: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("chain_broadcast.log")
        .unwrap();
    writeln!(file, "{}", txn).unwrap();
}

pub fn get_latest_processor_output() -> Option<String> {
    let last = LAST_OUTPUT.lock().unwrap();
    last.clone()
}
