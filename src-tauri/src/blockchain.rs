use crate::block::ProduceBatch;
use crate::merkle::calculate_merkle_root;
use chrono::Utc;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{BufReader, Write};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: String,
    pub data: Vec<ProduceBatch>,
    pub merkle_root: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let mut chain = Blockchain { chain: vec![], difficulty };
        chain.chain.push(chain.create_genesis_block());
        chain
    }

    fn create_genesis_block(&self) -> Block {
        let data = vec![ProduceBatch {
            batch_id: "GENESIS".into(),
            name: "Genesis Block".into(),
            origin: "System".into(),
            harvest_date: "N/A".into(),
            certifier: "System".into(),
        }];
        let merkle_root = calculate_merkle_root(&data);
        let timestamp = Utc::now().to_rfc3339();
        let (nonce, hash) = self.proof_of_work(0, &timestamp, &data, "0", &merkle_root);
        Block {
            index: 0,
            timestamp,
            data,
            merkle_root,
            previous_hash: "0".into(),
            hash,
            nonce,
        }
    }

    pub fn add_block(&mut self, data: Vec<ProduceBatch>) {
        let index = self.chain.len() as u64;
        let timestamp = Utc::now().to_rfc3339();
        let previous_hash = self.chain.last().unwrap().hash.clone();
        let merkle_root = calculate_merkle_root(&data);
        let (nonce, hash) = self.proof_of_work(index, &timestamp, &data, &previous_hash, &merkle_root);
        let block = Block {
            index,
            timestamp,
            data,
            merkle_root,
            previous_hash,
            hash,
            nonce,
        };
        self.chain.push(block);
    }

    pub fn proof_of_work(&self, index: u64, timestamp: &str, data: &[ProduceBatch], previous_hash: &str, merkle_root: &str) -> (u64, String) {
        let mut nonce = 0;
        loop {
            let input = format!("{}{}{:?}{}{}{}", index, timestamp, data, previous_hash, merkle_root, nonce);
            let hash = Sha256::digest(input.as_bytes());
            let hash_str = format!("{:x}", hash);
            if hash_str.starts_with(&"0".repeat(self.difficulty)) {
                return (nonce, hash_str);
            }
            nonce += 1;
        }
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];
            let input = format!(
                "{}{}{:?}{}{}{}",
                current.index,
                current.timestamp,
                current.data,
                current.previous_hash,
                current.merkle_root,
                current.nonce
            );
            let hash = Sha256::digest(input.as_bytes());
            let hash_str = format!("{:x}", hash);
            if current.hash != hash_str || current.previous_hash != previous.hash {
                return false;
            }
        }
        true
    }

    pub fn save(&self) {
        let json = serde_json::to_string_pretty(&self.chain).unwrap();
        let mut file = File::create("chain.json").unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }

    pub fn load(difficulty: usize) -> Self {
        let file = File::open("chain.json").unwrap_or_else(|_| return Blockchain::new(difficulty));
        let reader = BufReader::new(file);
        let chain: Vec<Block> = serde_json::from_reader(reader).unwrap_or_else(|_| vec![]);
        Blockchain { chain, difficulty }
    }
}