
use std::collections::{HashMap, HashSet};
use std::net::{SocketAddr};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use warp::Filter;
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use uuid::Uuid;
use sha3::{Digest, Sha3_256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub id: String,
    pub height: u64,
    pub transactions: Vec<String>,
    pub producer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GossipMessage {
    pub from: String,
    pub peers: Vec<String>,
}

#[derive(Clone)]
pub struct NetworkState {
    pub peers: Arc<Mutex<HashSet<String>>>,
    pub block_tx: broadcast::Sender<Block>,
    pub heartbeats: Arc<Mutex<HashMap<String, u128>>>,
    pub uptime: Arc<Mutex<HashMap<String, u128>>>,
}

pub async fn run_network(local_addr: &str, port: u16) {
    let peers = Arc::new(Mutex::new(HashSet::new()));
    let heartbeats = Arc::new(Mutex::new(HashMap::new()));
    let uptime = Arc::new(Mutex::new(HashMap::new()));
    let (block_tx, _) = broadcast::channel::<Block>(100);

    let state = NetworkState {
        peers: peers.clone(),
        block_tx: block_tx.clone(),
        heartbeats: heartbeats.clone(),
        uptime: uptime.clone(),
    };

    // Heartbeat + uptime update
    let beat_state = state.clone();
    tokio::spawn(async move {
        loop {
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            let id = "validator_001".to_string();
            {
                let mut beats = beat_state.heartbeats.lock().unwrap();
                beats.insert(id.clone(), now);
            }
            {
                let mut up = beat_state.uptime.lock().unwrap();
                let counter = up.entry(id.clone()).or_insert(0);
                *counter += 1;
                if *counter % 10 == 0 {
                    println!("[REWARD] {} earned uptime reward for {}s online", id, counter);
                }
            }
            println!("[HEARTBEAT] {}: {}ms", id, now);
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });

    // Simulate auto peer discovery
    let discover_state = state.clone();
    tokio::spawn(async move {
        loop {
            let mut known = discover_state.peers.lock().unwrap();
            known.insert("peer1.local:9000".to_string());
            known.insert("peer2.local:9000".to_string());
            println!("[DISCOVERY] Peers: {:?}", *known);
            tokio::time::sleep(Duration::from_secs(10)).await;
        }
    });

    // Live fanout broadcast
    let fanout_state = state.clone();
    tokio::spawn(async move {
        loop {
            let block = Block {
                id: Uuid::new_v4().to_string(),
                height: rand::random::<u64>() % 10000,
                transactions: vec!["tx_fanout_001".into()],
                producer: "validator_001".into(),
            };
            fanout_state.block_tx.send(block.clone()).unwrap();
            println!("[FANOUT] Broadcasted block {} to peers", block.id);
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    });

    // Simulated P2P messaging
    let p2p_state = state.clone();
    tokio::spawn(async move {
        loop {
            let peers = p2p_state.peers.lock().unwrap().clone();
            for peer in peers {
                println!("[P2P] Sent sync message to {}", peer);
                // Simulated message - real network call would go here
            }
            tokio::time::sleep(Duration::from_secs(15)).await;
        }
    });

    println!("[NETWORK] Fully initialized validator P2P stack.");
}
