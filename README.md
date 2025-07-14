🌾 Agri SelfChain  
Agri SelfChain 是一個以 Rust 建構的農業區塊鏈系統，支援批次資料上鏈、查詢、驗證與桌面操作介面。透過 Merkle Tree 與 Proof of Work 機制，確保農產品資料的完整性與不可竄改性，適用於農民、驗證者與消費者的履歷查驗需求。

🚀 功能特色
- ✅ 批次資料上鏈（含 PoW 雜湊驗證）
- ✅ Merkle Tree 根雜湊保護資料完整性
- ✅ CLI 模式快速輸入與出鏈
- ✅ API 模式提供 RESTful 查詢介面
- ✅ Tauri 桌面 GUI 操作介面
- ✅ 鏈合法性驗證機制
- ✅ 資料匯出為 CSV 檔案

📦 資料結構
ProduceBatch {
  batch_id: String,
  name: String,
  origin: String,
  harvest_date: String,
  certifier: String,
}


每個區塊可包含多筆 ProduceBatch，並計算 Merkle Root。

🖥️ 執行方式
🔹 CLI 模式
cargo run --bin cli


輸入農產品資料並出鏈，資料儲存在 chain.json

🔹 API 模式
cargo run --bin api


啟動 Actix Web 伺服器，支援查詢路由：
- /batch/{id}：查詢指定批次資料
- /verify（可擴充）：檢查鏈合法性

🔹 GUI 模式（Tauri 桌面應用）
cargo run --bin tauri


或使用：
tauri dev


開啟桌面視窗，支援：
- 表單輸入資料並出鏈
- 查詢批次資料
- 檢查鏈合法性
- （可擴充）匯出 CSV、掃描 QR

📄 匯出資料
執行 CLI 或 API 時可匯出鏈資料為 CSV：
chain.export_csv("chain_export.csv");


欄位包含：block_index, timestamp, batch_id, name, origin, harvest_date, certifier, merkle_root

🔐 鏈合法性驗證
每個區塊包含：
- previous_hash：前一區塊雜湊
- hash：本區塊雜湊（含 Merkle Root）
- nonce：PoW 驗證值
透過 is_valid() 函式可偵測任何資料竄改。

🔜 未來擴充方向
- 🔏 加入 ed25519 驗證者簽章
- 📤 匯出 Merkle Proof
- 📱 QR 查詢功能
- 🌐 公鏈同步（Solana / Ethereum）
- 📊 統計分析與視覺化
- 🧪 單元測試與鏈安全性測試

📁 專案結構摘要
src/
├── block.rs          # 資料結構
├── blockchain.rs     # 區塊鏈邏輯
├── merkle.rs         # Merkle Tree
├── cli_main.rs       # CLI 模式
├── api_main.rs       # API 模式
├── tauri_main.rs     # GUI 模式

src-tauri/
└── frontend/         # HTML/CSS/JS 桌面介面



🙌 作者
Win — 專注於 Rust、區塊鏈、AI 農業應用與 NFT 遊戲設計
📍 Pingtung, Taiwan
