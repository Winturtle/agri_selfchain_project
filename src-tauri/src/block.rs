use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProduceBatch {
    pub batch_id: String,
    pub name: String,
    pub origin: String,
    pub harvest_date: String,
    pub certifier: String,
}