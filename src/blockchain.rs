use crate::block::{Block, ProduceBatch};
use serde::{Serialize, Deserialize};
use std::fs::{File};
use std::io::{BufReader, Write};
use sha2::{Sha256, Digest};

#[derive(Serialize, Deserialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let genesis_data = ProduceBatch {
            batch_id: "GENESIS".to_string(),
            name: "Genesis Product".to_string(),
            origin: "N/A".to_string(),
            harvest_date: "N/A".to_string(),
            certifier: "System".to_string(),
        };
        let genesis = Block::new(0, genesis_data, "0".to_string(), difficulty);
        Blockchain {
            chain: vec![genesis],
            difficulty,
        }
    }

    pub fn add_block(&mut self, data: ProduceBatch) {
        let last = self.chain.last().unwrap();
        let block = Block::new(
            last.index + 1,
            data,
            last.hash.clone(),
            self.difficulty,
        );
        self.chain.push(block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            let input = format!(
                "{}{}{:?}{}{}",
                current.index,
                current.timestamp,
                current.data,
                current.previous_hash,
                current.nonce
            );

            let result = Sha256::digest(input.as_bytes());
            let hash_check = format!("{:x}", result);

            if current.hash != hash_check || current.previous_hash != previous.hash {
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
        if let Ok(file) = File::open("chain.json") {
            let reader = BufReader::new(file);
            if let Ok(chain) = serde_json::from_reader(reader) {
                return Blockchain { chain, difficulty };
            }
        }

        Blockchain::new(difficulty)
    }
}