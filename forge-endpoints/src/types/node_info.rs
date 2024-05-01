use serde::{Deserialize, Serialize};

use super::chain_parameters::Parameters;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NodeInfo {
    pub name: String,
    #[serde(rename = "appVersion")]
    pub app_version: String,
    #[serde(rename = "fullHeight")]
    pub full_height: u32,
    #[serde(rename = "headersHeight")]
    pub headers_height: u32,
    #[serde(rename = "maxPeerHeight")]
    pub max_peer_height: u32,
    #[serde(rename = "bestFullHeaderId")]
    pub best_full_header_id: String,
    #[serde(rename = "previousFullHeaderId")]
    pub previous_full_header_id: String,
    #[serde(rename = "bestHeaderId")]
    pub best_header_id: String,
    #[serde(rename = "stateRoot")]
    pub state_root: String,
    #[serde(rename = "stateType")]
    pub state_type: String,
    #[serde(rename = "stateVersion")]
    pub state_version: String,
    #[serde(rename = "isMining")]
    pub is_mining: bool,
    #[serde(rename = "peersCount")]
    pub peers_count: u32,
    #[serde(rename = "unconfirmedCount")]
    pub unconfirmed_count: u32,
    pub difficulty: u64,
    #[serde(rename = "currentTime")]
    pub current_time: u64,
    #[serde(rename = "launchTime")]
    pub launch_time: u64,
    #[serde(rename = "headersScore")]
    pub headers_score: f64,
    #[serde(rename = "fullBlocksScore")]
    pub full_blocks_score: f64,
    #[serde(rename = "genesisBlockId")]
    pub genesis_block_id: String,
    pub parameters: Parameters,
    #[serde(rename = "eip27Supported")]
    pub eip27_supported: bool,
    #[serde(rename = "restApiUrl")]
    pub rest_api_url: String,
}