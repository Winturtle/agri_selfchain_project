use chrono::Utc;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProduceBatch {
    pub batch_id: String,
    pub name: String,
    pub origin: String,
    pub harvest_date: String,
    pub certifier: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: String,
    pub data: ProduceBatch,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u64, data: ProduceBatch, previous_hash: String, difficulty: usize) -> Self {
        let timestamp = Utc::now().to_rfc3339();
        let mut nonce = 0;
        let mut hash = String::new();

        loop {
            let input = format!("{index}{timestamp}{:?}{previous_hash}{nonce}", data);
            let result = Sha256::digest(input.as_bytes());
            hash = format!("{:x}", result);

            if hash.starts_with(&"0".repeat(difficulty)) {
                break;
            }

            nonce += 1;
        }

        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
            nonce,
        }
    }
}