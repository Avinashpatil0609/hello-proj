use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize,Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Student {
    pub(crate) id: String,
    name: String,
}
pub type Storage= Arc<RwLock<Vec<Student>>>;

pub fn init_store() -> Storage {
    // let store = Store::new();
    // let store_filter = warp::any().map(move || store.clone());

    return Arc::new(RwLock::new(Vec::new()))
}