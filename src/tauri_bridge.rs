#[tauri::command]
pub fn add_batch(batch: ProduceBatch) -> bool {
    let mut chain = Blockchain::load(4);
    chain.add_block(vec![batch]);
    chain.save();
    true
}

#[tauri::command]
pub fn query_batch(batch_id: String) -> Option<ProduceBatch> {
    let chain = Blockchain::load(4);
    for block in chain.chain {
        for data in block.data {
            if data.batch_id == batch_id {
                return Some(data);
            }
        }
    }
    None
}

#[tauri::command]
pub fn verify_chain() -> bool {
    let chain = Blockchain::load(4);
    chain.is_valid()
}