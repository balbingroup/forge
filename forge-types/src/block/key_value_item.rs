use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyValueItem {
    pub key: String,
    pub value: String,
}

impl KeyValueItem {
    pub fn new(key: String, value: String) -> Self {
        Self { key, value }
    }
}