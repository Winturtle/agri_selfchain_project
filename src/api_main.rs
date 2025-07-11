mod block;
mod blockchain;
mod api;

use actix_web::{App, HttpServer};
use api::get_batch;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 啟動區塊鏈查詢 API at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .service(get_batch)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}