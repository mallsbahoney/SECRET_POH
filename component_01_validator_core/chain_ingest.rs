
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use sha3::{Digest, Sha3_256};
use std::collections::HashMap;
use serde_json::Value;

pub fn read_broadcast_log() {
    if let Ok(file) = File::open("chain_broadcast.log") {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(txn) = line {
                if txn.starts_with("balance_hash|") {
                    let parts: Vec<&str> = txn.split('|').collect();
                    if parts.len() == 2 {
                        let claimed_hash = parts[1];
                        let valid = verify_local_balance_hash(claimed_hash);
                        if valid {
                            println!("[VALIDATOR] Hash match confirmed: {}", claimed_hash);
                        } else {
                            println!("[FATAL] Hash mismatch! Aborting validator to prevent fork.");
                            std::process::exit(1);
                        }
                    }
                } else {
                    println!("[CHAIN] TX: {}", txn);
                }
            }
        }
    }
}

fn verify_local_balance_hash(expected_hash: &str) -> bool {
    if let Ok(mut file) = File::open("user_balances.json") {
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        let mut hasher = Sha3_256::new();
        hasher.update(&buf);
        let actual = hex::encode(hasher.finalize());
        return actual == expected_hash;
    }
    false
}

pub fn gossip_balance_state_to(peers: &[String]) {
    let balances = if let Ok(file) = File::open("user_balances.json") {
        let map: HashMap<String, u64> = serde_json::from_reader(file).unwrap_or_default();
        map
    } else {
        HashMap::new()
    };
    let hash = {
        let mut hasher = Sha3_256::new();
        let serialized = serde_json::to_string(&balances).unwrap_or_default();
        hasher.update(serialized.as_bytes());
        hex::encode(hasher.finalize())
    };

    for peer in peers {
        println!("[GOSSIP] Sending balance hash {} to peer {}", hash, peer);
        // Simulated send - would replace with real P2P
    }

    // Optional simulated sync fallback
    let fallback = "peer_hashes.json";
    if let Ok(file) = File::open(fallback) {
        let peer_hashes: HashMap<String, String> = serde_json::from_reader(file).unwrap_or_default();
        for (peer, their_hash) in peer_hashes {
            if !verify_local_balance_hash(&their_hash) {
                println!("[SYNC] Hash mismatch with {}, pulling their version...", peer);
                // Simulated: fetch and write new version
                if let Ok(content) = std::fs::read_to_string(format!("{}_balances.json", peer)) {
                    std::fs::write("user_balances.json", content).unwrap();
                    println!("[SYNC] Synced from peer {}", peer);
                }
            }
        }
    }
}
