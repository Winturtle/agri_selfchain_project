use crate::block::ProduceBatch;
use sha2::{Sha256, Digest};

pub fn calculate_merkle_root(data: &[ProduceBatch]) -> String {
    let mut hashes: Vec<String> = data.iter()
        .map(|d| {
            let input = format!("{:?}", d);
            let hash = Sha256::digest(input.as_bytes());
            format!("{:x}", hash)
        })
        .collect();

    while hashes.len() > 1 {
        let mut next_level = vec![];
        for i in (0..hashes.len()).step_by(2) {
            let left = &hashes[i];
            let right = if i + 1 < hashes.len() { &hashes[i + 1] } else { left };
            let combined = format!("{}{}", left, right);
            let hash = Sha256::digest(combined.as_bytes());
            next_level.push(format!("{:x}", hash));
        }
        hashes = next_level;
    }

    hashes[0].clone()
}