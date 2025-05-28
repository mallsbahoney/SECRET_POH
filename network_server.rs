
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use warp::Filter;
use uuid::Uuid;
use tokio::sync::broadcast;
use tokio_tungstenite::tungstenite::Message;
use warp::ws::{Ws, WebSocket};
use futures_util::{SinkExt, StreamExt};
use rand::{Rng, distributions::Alphanumeric};

#[derive(Clone)]
pub struct LiveValidatorState {
    pub peers: Arc<Mutex<HashSet<String>>>,
    pub uptime: Arc<Mutex<HashMap<String, u128>>>,
    pub rewards: Arc<Mutex<HashMap<String, u64>>>,
    pub keys: Arc<Mutex<HashMap<String, String>>>,
}

pub async fn start_live_validator_stack() {
    let state = LiveValidatorState {
        peers: Arc::new(Mutex::new(HashSet::new())),
        uptime: Arc::new(Mutex::new(HashMap::new())),
        rewards: Arc::new(Mutex::new(HashMap::new())),
        keys: Arc::new(Mutex::new(HashMap::new())),
    };

    // Spawn uptime tracking + reward logic
    let reward_state = state.clone();
    tokio::spawn(async move {
        loop {
            let mut up = reward_state.uptime.lock().unwrap();
            let mut rewards = reward_state.rewards.lock().unwrap();
            for (validator, time) in up.iter_mut() {
                *time += 1;
                if *time % 10 == 0 {
                    *rewards.entry(validator.clone()).or_insert(0) += 1;
                    println!("[REWARD] {} rewarded! Total: {}", validator, rewards[validator]);
                }
            }
            drop(up);
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });

    // WebSocket handler
    let ws_state = state.clone();
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::any().map(move || ws_state.clone()))
        .map(|ws: Ws, state: LiveValidatorState| {
            ws.on_upgrade(move |socket| handle_ws_connection(socket, state))
        });

    // REST API for uptime/rewards
    let explorer_state = state.clone();
    let explorer = warp::path("explorer")
        .map(move || {
            let u = explorer_state.uptime.lock().unwrap();
            let r = explorer_state.rewards.lock().unwrap();
            let map: HashMap<_, _> = u.iter().map(|(k, v)| {
                let rewards = r.get(k).cloned().unwrap_or(0);
                (k.clone(), (v.clone(), rewards))
            }).collect();
            warp::reply::json(&map)
        });

    let routes = ws_route.or(explorer);
    println!("[NETWORK] Live P2P and explorer running on ws://localhost:9000/ws and /explorer");
    warp::serve(routes).run(([127, 0, 0, 1], 9000)).await;
}

async fn handle_ws_connection(ws: WebSocket, state: LiveValidatorState) {
    let (mut sender, mut receiver) = ws.split();
    let id = Uuid::new_v4().to_string();
    let key = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect::<String>();

    {
        let mut peers = state.peers.lock().unwrap();
        let mut keys = state.keys.lock().unwrap();
        peers.insert(id.clone());
        keys.insert(id.clone(), key.clone());
        println!("[P2P] Validator {} joined with key {}", id, key);
    }

    let uptime = state.uptime.clone();
    tokio::spawn(async move {
        while let Some(result) = receiver.next().await {
            if let Ok(msg) = result {
                if msg.is_text() {
                    println!("[P2P] {} received: {}", id, msg.to_text().unwrap());
                    let mut up = uptime.lock().unwrap();
                    up.insert(id.clone(), *up.get(&id).unwrap_or(&0));
                }
            }
        }
    });

    // Echo welcome
    let _ = sender.send(Message::text(format!("Welcome, validator {}!", id))).await;
}
