
use std::collections::HashMap;

pub struct Shard {
    pub index: usize,
    pub data: Vec<u8>,
}

pub fn shard_task(data: &[u8], shard_size: usize) -> Vec<Shard> {
    data.chunks(shard_size)
        .enumerate()
        .map(|(i, chunk)| Shard { index: i, data: chunk.to_vec() })
        .collect()
}

pub fn assign_shards_to_nodes(shards: Vec<Shard>, available_nodes: Vec<String>) -> HashMap<String, Vec<Shard>> {
    let mut assignments = HashMap::new();
    for (i, shard) in shards.into_iter().enumerate() {
        let node = &available_nodes[i % available_nodes.len()];
        assignments.entry(node.clone()).or_insert(vec![]).push(shard);
    }
    assignments
}
