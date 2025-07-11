use actix_web::{get, App, HttpServer, Responder, web, HttpResponse};
use crate::blockchain::Blockchain;
use crate::block::ProduceBatch;
use serde_json::json;

#[get("/batch/{id}")]
pub async fn get_batch(id: web::Path<String>) -> impl Responder {
    let batch_id = id.into_inner(); // ✅ 拿出真正的 String
    let chain = Blockchain::load(4);

    for block in chain.chain {
        if block.data.batch_id == batch_id {
            return HttpResponse::Ok().json(block.data);
        }
    }

    HttpResponse::NotFound().json(json!({ "error": "Batch not found" }))
}