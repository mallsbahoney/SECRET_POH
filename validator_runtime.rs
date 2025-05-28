// Rewritten validator_runtime.rs for full POH parallelism
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use std::collections::VecDeque;
use std::sync::mpsc::{channel, Sender, Receiver};

use crate::schemas::*;
use crate::state::*;

pub struct Task {
    pub schema_id: String,
    pub payload: serde_json::Value,
    pub tick: u64,
}

pub struct Validator {
    pub tick: Arc<Mutex<u64>>,
    pub task_queues: Arc<Mutex<VecDeque<Task>>>,
    pub reward_tracker: Arc<Mutex<u64>>,
}

impl Validator {
    pub fn new() -> Self {
        Self {
            tick: Arc::new(Mutex::new(0)),
            task_queues: Arc::new(Mutex::new(VecDeque::new())),
            reward_tracker: Arc::new(Mutex::new(0)),
        }
    }

    pub fn submit_task(&self, task: Task) {
        let mut tq = self.task_queues.lock().unwrap();
        tq.push_back(task);
    }

    pub fn start(&self) {
        let tick_clone = self.tick.clone();
        let queue_clone = self.task_queues.clone();
        let reward_clone = self.reward_tracker.clone();

        // 1ms global clock
        thread::spawn(move || {
            loop {
                {
                    let mut t = tick_clone.lock().unwrap();
                    *t += 1;
                }
                thread::sleep(Duration::from_millis(1));
            }
        });

        // Schema-routed execution loop (multithreaded)
        for _ in 0..8 {
            let queue_clone = queue_clone.clone();
            let reward_clone = reward_clone.clone();
            let tick_clone = tick_clone.clone();

            thread::spawn(move || {
                loop {
                    let task_opt = {
                        let mut queue = queue_clone.lock().unwrap();
                        queue.pop_front()
                    };

                    if let Some(task) = task_opt {
                        let schema = get_schema_by_id(&task.schema_id);
                        let is_valid = schema.validate(&task.payload);

                        if is_valid {
                            if task.payload.to_string().len() > 100000 {
    dispatch_to_best_processor(&task.payload, task.payload.to_string().len());
} else {
    schema.execute(&task.payload);
}
                            let mut rewards = reward_clone.lock().unwrap();
                            *rewards += 1;
                            println!("[EXECUTED @ TICK {}] {}", task.tick, task.schema_id);
                        } else {
                            println!("[REJECTED] {}", task.schema_id);
                        }
                    }

                    thread::sleep(Duration::from_millis(1));
                }
            });
        }
    }
}
