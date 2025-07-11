mod block;
mod blockchain;

use std::io::{self, Write};
use block::ProduceBatch;
use blockchain::Blockchain;
use actix_web::{App, HttpServer};

fn main() {
    // 設定工作量證明難度（例如開頭 4 個 0）
    let mut chain = Blockchain::load(4);

    println!("🌱 歡迎使用農業區塊鏈！請輸入批次資料（輸入空白 batch_id 結束）：");

    loop {
        let mut batch_id = String::new();
        print!("🔢 批次編號：");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut batch_id).unwrap();
        let batch_id = batch_id.trim().to_string();
        if batch_id.is_empty() { break; }

        let name = prompt("📦 品名：");
        let origin = prompt("🌍 產地：");
        let harvest_date = prompt("📆 採收日（YYYY-MM-DD）：");
        let certifier = prompt("🔒 認證單位：");

        let batch = ProduceBatch {
            batch_id,
            name,
            origin,
            harvest_date,
            certifier,
        };

        println!("⛏️ 正在出鏈...");
        chain.add_block(batch);
    }
	chain.save();
	println!("💾 鏈已儲存至 chain.json");

    for block in &chain.chain {
        println!("{:#?}", block);
	}
}
fn prompt(label: &str) -> String {
    let mut input = String::new();
    print!("{}", label);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
