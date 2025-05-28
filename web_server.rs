
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use tokio::spawn;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicUsize, Ordering};
use rand::Rng;
use std::sync::Arc;
use component_02_stateless_engine::modules::processor_profile::{get_latest_processor_output, execute_processor_task};

static FRAME_MODE: AtomicUsize = AtomicUsize::new(0); // 0 = JSON, 1 = frame

pub async fn start_websocket_server() {
    let addr = "0.0.0.0:9001".parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("WebSocket server started on ws://{}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        spawn(async move {
            let ws_stream = accept_async(stream).await.unwrap();
            let (mut write, mut read) = ws_stream.split();

            // Set mode
            if let Some(Ok(msg)) = read.next().await {
                if let Ok(text) = msg.to_text() {
                    if text.trim() == "frame" {
                        FRAME_MODE.store(1, Ordering::SeqCst);
                    } else {
                        FRAME_MODE.store(0, Ordering::SeqCst);
                    }
                }
            }

            // Spawn task for receiving inputs
            let mut reader = read;
            let writer = Arc::new(tokio::sync::Mutex::new(write));
            let writer_clone = writer.clone();
            spawn(async move {
                while let Some(Ok(msg)) = reader.next().await {
                    if let Ok(input) = msg.to_text() {
                        if FRAME_MODE.load(Ordering::SeqCst) == 0 && input != "json" && input != "frame" {
                            let _ = execute_processor_task(input);
                        }
                    }
                }
            });

            // Output task stream
            loop {
                let msg = if FRAME_MODE.load(Ordering::SeqCst) == 0 {
                    if let Some(real) = get_latest_processor_output() {
                        real
                    } else {
                        serde_json::json!({
                            "task": "idle",
                            "status": "waiting",
                            "timestamp": chrono::Utc::now().timestamp_millis()
                        }).to_string()
                    }
                } else {
                    let width = 100;
                    let height = 10;
                    let pixels: Vec<u8> = (0..(width * height))
                        .map(|_| rand::thread_rng().gen_range(0..255))
                        .collect();
                    serde_json::json!({
                        "type": "frame",
                        "timestamp": chrono::Utc::now().timestamp_millis(),
                        "width": width,
                        "height": height,
                        "pixels": pixels
                    }).to_string()
                };

                let mut w = writer_clone.lock().await;
                if w.send(tokio_tungstenite::tungstenite::Message::Text(msg)).await.is_err() {
                    break;
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
            }
        });
    }
}
