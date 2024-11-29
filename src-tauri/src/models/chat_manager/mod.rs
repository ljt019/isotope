pub mod chat_manager;

use serde::{Deserialize, Serialize};

pub use chat_manager::ChatManager;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}
