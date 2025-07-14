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
	pub fn export_csv(&self, path: &str) {
        let mut file = File::create(path).expect("無法建立 CSV 檔案");
        writeln!(
            file,
            "block_index,timestamp,batch_id,name,origin,harvest_date,certifier,merkle_root"
        ).unwrap();

        for block in &self.chain {
            for batch in &block.data {
                writeln!(
                    file,
                    "{},{},{},{},{},{},{},{}",
                    block.index,
                    block.timestamp,
                    batch.batch_id,
                    batch.name,
                    batch.origin,
                    batch.harvest_date,
                    batch.certifier,
                    block.merkle_root
                ).unwrap();
            }
        }
    }

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

    pub fn add_block(&mut self, data: Vec<ProduceBatch>) {
    let index = self.chain.len() as u64;
    let timestamp = Utc::now().to_rfc3339();
    let previous_hash = self.chain.last().unwrap().hash.clone();
    let merkle_root = calculate_merkle_root(&data);

    // 雜湊計算包含 merkle_root
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