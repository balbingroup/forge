use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndexedHeightResponse {
    #[serde(rename = "indexedHeight")]
    pub indexed_height: u64,
    #[serde(rename = "fullHeight")]
    pub full_height: u64,
}

impl IndexedHeightResponse {
    pub fn new(indexed_height: u64, full_height: u64) -> Self {
        Self {
            indexed_height,
            full_height,
        }
    }
}